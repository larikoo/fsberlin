// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The SQLite index mirror (`.fsberlin/index.sqlite`). It is a **cache, never
//! a store** (ADR-001, ADR-002): every `rebuild` drops and repopulates the
//! tables from the filesystem, so the filesystem is always authoritative and
//! the index is fully regenerable.

use std::path::Path;

use rusqlite::{Connection, params};

use crate::Result;
use crate::links;
use crate::model::Card;

/// The index schema. Mirrors the queryable frontmatter fields plus a flat
/// relations table. Recreated on every [`rebuild`].
pub const SCHEMA: &str = "\
CREATE TABLE cards (
    slug            TEXT PRIMARY KEY,
    id              TEXT NOT NULL,
    title           TEXT NOT NULL,
    type            TEXT NOT NULL,
    created         TEXT NOT NULL,
    priority        TEXT,
    phase           INTEGER,
    building_status TEXT,
    planning_status TEXT,
    waypoint_status TEXT,
    adr_number      INTEGER,
    phase_number    INTEGER
);
CREATE TABLE relations (
    from_slug TEXT NOT NULL,
    kind      TEXT NOT NULL,
    to_slug   TEXT NOT NULL
);
";

/// Open (creating if needed) an index database at `path`.
pub fn open(path: &Path) -> Result<Connection> {
    Ok(Connection::open(path)?)
}

/// Rebuild the index from the filesystem rooted at `root`, dropping any prior
/// contents first. Returns the number of cards mirrored.
///
/// Because this drops and repopulates, the index can never drift: stale rows
/// are discarded and the filesystem wins.
pub fn rebuild(conn: &Connection, root: &Path) -> Result<usize> {
    let resolution = links::resolve(root)?;

    conn.execute_batch("DROP TABLE IF EXISTS relations; DROP TABLE IF EXISTS cards;")?;
    conn.execute_batch(SCHEMA)?;

    let tx = conn.unchecked_transaction()?;
    for (slug, card) in &resolution.by_slug {
        insert_card(conn, slug, card)?;
        let relations = [
            ("depends_on", card.depends_on()),
            ("blocks", card.blocks()),
            ("linked", card.linked()),
            ("criteria", card.criteria()),
        ];
        for (kind, targets) in relations {
            for to in targets {
                conn.execute(
                    "INSERT INTO relations (from_slug, kind, to_slug) VALUES (?1, ?2, ?3)",
                    params![slug, kind, to],
                )?;
            }
        }
    }
    tx.commit()?;

    Ok(resolution.by_slug.len())
}

/// Convenience: build a fresh in-memory index from `root` (used by queries
/// and tests).
pub fn build_in_memory(root: &Path) -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    rebuild(&conn, root)?;
    Ok(conn)
}

fn insert_card(conn: &Connection, slug: &str, card: &Card) -> Result<()> {
    let (building, planning, waypoint_status, adr_number, phase_number) = match card {
        Card::Work(c) => (Some(c.building_status.as_str()), None, None, None, None),
        Card::Adr(c) => (
            None,
            Some(c.planning_status.as_str()),
            None,
            Some(i64::from(c.adr_number)),
            None,
        ),
        Card::Phase(c) => (None, None, None, None, Some(i64::from(c.phase_number))),
        Card::Waypoint(c) => (None, None, Some(c.status.as_str()), None, None),
    };
    conn.execute(
        "INSERT INTO cards \
         (slug, id, title, type, created, priority, phase, building_status, \
          planning_status, waypoint_status, adr_number, phase_number) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            slug,
            card.id(),
            card.title(),
            card.card_type().as_str(),
            card.created(),
            card.priority().map(|p| p.as_str()),
            card.phase(),
            building,
            planning,
            waypoint_status,
            adr_number,
            phase_number,
        ],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn card(root: &Path, slug: &str, body: &str) {
        let p = root.join("cards").join(slug).join("card.md");
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, format!("---\n{body}\n---\n")).unwrap();
    }

    /// A deterministic textual dump of the index contents.
    fn dump(conn: &Connection) -> Vec<String> {
        let mut out = Vec::new();
        let mut cards = conn
            .prepare("SELECT slug, type, building_status, planning_status FROM cards ORDER BY slug")
            .unwrap();
        let rows = cards
            .query_map([], |r| {
                Ok(format!(
                    "card {} {} {:?} {:?}",
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, Option<String>>(2)?,
                    r.get::<_, Option<String>>(3)?,
                ))
            })
            .unwrap();
        for row in rows {
            out.push(row.unwrap());
        }
        let mut rels = conn
            .prepare(
                "SELECT from_slug, kind, to_slug FROM relations ORDER BY from_slug, kind, to_slug",
            )
            .unwrap();
        let rows = rels
            .query_map([], |r| {
                Ok(format!(
                    "rel {} {} {}",
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                ))
            })
            .unwrap();
        for row in rows {
            out.push(row.unwrap());
        }
        out
    }

    fn sample(root: &Path) {
        card(
            root,
            "a",
            "id: 01a\ntitle: A\ntype: card\nbuilding_status: in-progress\ncreated: 2026-05-29\ndepends_on: [b]",
        );
        card(
            root,
            "b",
            "id: 01b\ntitle: B\ntype: card\nbuilding_status: done\ncreated: 2026-05-29",
        );
    }

    #[test]
    fn builds_expected_rows() {
        let dir = tempdir().unwrap();
        sample(dir.path());
        let conn = build_in_memory(dir.path()).unwrap();

        let n: i64 = conn
            .query_row("SELECT count(*) FROM cards", [], |r| r.get(0))
            .unwrap();
        assert_eq!(n, 2);
        let dep: String = conn
            .query_row(
                "SELECT to_slug FROM relations WHERE from_slug='a' AND kind='depends_on'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(dep, "b");
        let status: String = conn
            .query_row(
                "SELECT building_status FROM cards WHERE slug='a'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(status, "in-progress");
    }

    #[test]
    fn rebuild_is_deterministic() {
        let dir = tempdir().unwrap();
        sample(dir.path());
        let first = dump(&build_in_memory(dir.path()).unwrap());
        let second = dump(&build_in_memory(dir.path()).unwrap());
        assert_eq!(first, second);
    }

    #[test]
    fn rebuild_discards_stale_rows_filesystem_wins() {
        let dir = tempdir().unwrap();
        sample(dir.path());
        let conn = build_in_memory(dir.path()).unwrap();
        conn.execute(
            "INSERT INTO cards (slug, id, title, type, created) \
             VALUES ('ghost', 'x', 'Ghost', 'card', '2026-05-29')",
            [],
        )
        .unwrap();

        rebuild(&conn, dir.path()).unwrap();

        let ghosts: i64 = conn
            .query_row("SELECT count(*) FROM cards WHERE slug='ghost'", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(ghosts, 0, "rebuild must drop rows not on the filesystem");
    }
}

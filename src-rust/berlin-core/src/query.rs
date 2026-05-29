// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A minimal query grammar over the index: `field:value` predicates joined by
//! `AND` / `OR`. Field names are validated against the index columns (so a
//! query can never inject SQL); values are always bound as parameters.
//!
//! Parentheses and `NOT` are intentionally out of scope for Phase 1; mixed
//! `AND`/`OR` follow SQL precedence (`AND` binds tighter).

use rusqlite::{Connection, params_from_iter};

use crate::{Error, Result};

/// Index columns that may appear as a query field.
const COLUMNS: &[&str] = &[
    "slug",
    "id",
    "title",
    "type",
    "created",
    "priority",
    "phase",
    "building_status",
    "planning_status",
    "waypoint_status",
    "adr_number",
    "phase_number",
];

/// Run `expr` against the card index, returning `(slug, title)` matches
/// ordered by slug.
pub fn run(conn: &Connection, expr: &str) -> Result<Vec<(String, String)>> {
    let (where_clause, values) = compile(expr)?;
    let sql = format!("SELECT slug, title FROM cards WHERE {where_clause} ORDER BY slug");
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params_from_iter(values.iter()), |r| {
        Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

/// Compile an expression into a parameterized `WHERE` clause and its bound
/// values.
fn compile(expr: &str) -> Result<(String, Vec<String>)> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    if tokens.is_empty() {
        return Err(Error::Query("empty query".into()));
    }

    let mut parts: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();
    let mut expect_predicate = true;

    for tok in tokens {
        if expect_predicate {
            let (field, value) = tok
                .split_once(':')
                .ok_or_else(|| Error::Query(format!("expected `field:value`, got `{tok}`")))?;
            if !COLUMNS.contains(&field) {
                return Err(Error::Query(format!("unknown field `{field}`")));
            }
            values.push(value.to_string());
            parts.push(format!("{field} = ?{}", values.len()));
            expect_predicate = false;
        } else {
            let connector = tok.to_ascii_uppercase();
            if connector != "AND" && connector != "OR" {
                return Err(Error::Query(format!("expected AND/OR, got `{tok}`")));
            }
            parts.push(connector);
            expect_predicate = true;
        }
    }

    if expect_predicate {
        return Err(Error::Query("query ends with a connector".into()));
    }
    Ok((parts.join(" "), values))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::index::build_in_memory;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    fn card(root: &Path, slug: &str, body: &str) {
        let p = root.join("cards").join(slug).join("card.md");
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, format!("---\n{body}\n---\n")).unwrap();
    }

    fn project() -> tempfile::TempDir {
        let dir = tempdir().unwrap();
        let root = dir.path();
        card(
            root,
            "w1",
            "id: 01w1\ntitle: Work one\ntype: card\nbuilding_status: in-progress\ncreated: 2026-05-29",
        );
        card(
            root,
            "w2",
            "id: 01w2\ntitle: Work two\ntype: card\nbuilding_status: done\ncreated: 2026-05-29",
        );
        card(
            root,
            "a1",
            "id: 01a1\ntitle: Decision\ntype: adr\nadr_number: 1\nplanning_status: accepted\ncreated: 2026-05-29",
        );
        dir
    }

    fn slugs(rows: &[(String, String)]) -> Vec<&str> {
        rows.iter().map(|(s, _)| s.as_str()).collect()
    }

    #[test]
    fn single_predicate() {
        let dir = project();
        let conn = build_in_memory(dir.path()).unwrap();
        let rows = run(&conn, "type:card").unwrap();
        assert_eq!(slugs(&rows), ["w1", "w2"]);
    }

    #[test]
    fn and_narrows() {
        let dir = project();
        let conn = build_in_memory(dir.path()).unwrap();
        let rows = run(&conn, "type:card AND building_status:in-progress").unwrap();
        assert_eq!(slugs(&rows), ["w1"]);
    }

    #[test]
    fn or_unions() {
        let dir = project();
        let conn = build_in_memory(dir.path()).unwrap();
        let rows = run(&conn, "type:adr OR building_status:done").unwrap();
        assert_eq!(slugs(&rows), ["a1", "w2"]);
    }

    #[test]
    fn unknown_field_is_rejected() {
        let dir = project();
        let conn = build_in_memory(dir.path()).unwrap();
        let err = run(&conn, "bogus:x").unwrap_err();
        assert!(matches!(err, Error::Query(_)));
        assert!(err.to_string().contains("bogus"));
    }

    #[test]
    fn malformed_predicate_is_rejected() {
        let dir = project();
        let conn = build_in_memory(dir.path()).unwrap();
        assert!(matches!(run(&conn, "no-colon"), Err(Error::Query(_))));
        assert!(matches!(run(&conn, "type:card AND"), Err(Error::Query(_))));
    }
}

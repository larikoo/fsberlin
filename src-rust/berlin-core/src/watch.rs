// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Keeping the SQLite index live as files change (ADR-007): a debounced,
//! atomic-rename-tolerant watcher that rebuilds the index after each settled
//! burst and tolerates transient broken parses — the previous index is kept
//! until the file parses again.

use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{RecvTimeoutError, channel};
use std::time::Duration;

use notify::{RecursiveMode, Watcher};
use rusqlite::Connection;

use crate::Result;
use crate::index;
use crate::walk::is_opaque;

/// Outcome of a tolerant rebuild.
#[derive(Debug, PartialEq, Eq)]
pub enum RebuildOutcome {
    /// Index rebuilt; holds the card count.
    Updated(usize),
    /// Rebuild skipped after a transient error; the prior index is intact.
    Skipped(String),
}

/// Rebuild the index, tolerating a transient parse/resolve failure.
///
/// On error the existing index is left untouched (ADR-007 §004): [`index::rebuild`]
/// resolves the whole project *before* mutating the database, so a file caught
/// mid-save never corrupts the index — it is simply skipped until it parses.
pub fn rebuild_tolerant(conn: &Connection, root: &Path) -> RebuildOutcome {
    match index::rebuild(conn, root) {
        Ok(n) => RebuildOutcome::Updated(n),
        Err(e) => RebuildOutcome::Skipped(e.to_string()),
    }
}

/// Watch `root`, keeping the index at `db_path` live. Blocks until `stop` is
/// set. `debounce` is the quiet period that coalesces editor save bursts.
pub fn watch(root: &Path, db_path: &Path, debounce: Duration, stop: &AtomicBool) -> Result<()> {
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let conn = index::open(db_path)?;
    let _ = rebuild_tolerant(&conn, root); // initial build; tolerate a broken file

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    })?;
    watcher.watch(root, RecursiveMode::Recursive)?;

    let poll = Duration::from_millis(200);
    while !stop.load(Ordering::Relaxed) {
        match rx.recv_timeout(poll) {
            Ok(event) => {
                if !is_relevant(&event) {
                    continue;
                }
                // Coalesce the burst: keep draining until quiet for `debounce`.
                while rx.recv_timeout(debounce).is_ok() {}
                let _ = rebuild_tolerant(&conn, root);
            }
            Err(RecvTimeoutError::Timeout) => {} // loop to re-check `stop`
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
    Ok(())
}

/// Whether an event touches a non-opaque path (ADR-008: ignore churn inside
/// `.git`, `.beads`, etc.). Watcher errors are treated as relevant so they
/// trigger a (harmless) rebuild rather than being silently dropped.
fn is_relevant(event: &notify::Result<notify::Event>) -> bool {
    match event {
        Ok(ev) => ev.paths.iter().any(|p| {
            !p.components()
                .any(|c| is_opaque(&c.as_os_str().to_string_lossy()))
        }),
        Err(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn card(root: &Path, slug: &str, body: &str) {
        let p = root.join("cards").join(slug).join("card.md");
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, format!("---\n{body}\n---\n")).unwrap();
    }

    fn count(conn: &Connection) -> i64 {
        conn.query_row("SELECT count(*) FROM cards", [], |r| r.get(0))
            .unwrap()
    }

    #[test]
    fn tolerant_skips_broken_parse_and_keeps_prior_index() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        card(
            root,
            "a",
            "id: 01a\ntitle: A\ntype: card\nbuilding_status: pending\ncreated: 2026-05-29",
        );
        card(
            root,
            "b",
            "id: 01b\ntitle: B\ntype: card\nbuilding_status: done\ncreated: 2026-05-29",
        );
        let conn = Connection::open_in_memory().unwrap();

        assert_eq!(rebuild_tolerant(&conn, root), RebuildOutcome::Updated(2));
        assert_eq!(count(&conn), 2);

        // Card caught mid-save: invalid YAML.
        fs::write(root.join("cards/a/card.md"), "---\nid: [broken\n---\n").unwrap();
        match rebuild_tolerant(&conn, root) {
            RebuildOutcome::Skipped(_) => {}
            other => panic!("expected Skipped, got {other:?}"),
        }
        // Prior index is intact — the broken file did not corrupt it.
        assert_eq!(count(&conn), 2);

        // Once it parses again, the index catches up.
        card(
            root,
            "a",
            "id: 01a\ntitle: A\ntype: card\nbuilding_status: review\ncreated: 2026-05-29",
        );
        assert_eq!(rebuild_tolerant(&conn, root), RebuildOutcome::Updated(2));
        let status: String = conn
            .query_row(
                "SELECT building_status FROM cards WHERE slug='a'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(status, "review");
    }
}

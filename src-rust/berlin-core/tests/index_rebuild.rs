// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration: the index mirrors the real projects, and rebuilding from the
//! filesystem reproduces identical contents (regenerable cache, ADR-001/002).

use std::path::{Path, PathBuf};

use berlin_core::index::build_in_memory;
use berlin_core::links::resolve;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn card_count(conn: &rusqlite::Connection) -> i64 {
    conn.query_row("SELECT count(*) FROM cards", [], |r| r.get(0))
        .unwrap()
}

#[test]
fn index_card_count_matches_resolution() {
    let root = repo_root().join("berlin-development");
    let conn = build_in_memory(&root).unwrap();
    let resolved = resolve(&root).unwrap();
    assert_eq!(card_count(&conn) as usize, resolved.by_slug.len());
    assert!(card_count(&conn) >= 25);
}

#[test]
fn rebuild_reproduces_identical_contents() {
    let root = repo_root().join("examples/sample-project");
    let a = build_in_memory(&root).unwrap();
    let b = build_in_memory(&root).unwrap();
    assert_eq!(card_count(&a), card_count(&b));

    // A fresh rebuild from the same filesystem yields the same relation rows.
    let rels = |c: &rusqlite::Connection| -> Vec<(String, String, String)> {
        let mut stmt = c
            .prepare("SELECT from_slug, kind, to_slug FROM relations ORDER BY 1, 2, 3")
            .unwrap();
        let rows = stmt
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
            .unwrap();
        rows.map(std::result::Result::unwrap).collect()
    };
    assert_eq!(rels(&a), rels(&b));
}

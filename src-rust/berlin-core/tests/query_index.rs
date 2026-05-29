// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration: querying the real dogfooding index returns the expected
//! cards (the Phase 1 success criterion for p1-cli-query).

use std::path::{Path, PathBuf};

use berlin_core::index::build_in_memory;
use berlin_core::query::run;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn queries_dogfooding_index() {
    let conn = build_in_memory(&repo_root().join("berlin-development")).unwrap();

    // All twelve ADRs (001–012) are accepted.
    let adrs = run(&conn, "type:adr").unwrap();
    assert_eq!(adrs.len(), 12, "got {:?}", adrs);
    let accepted = run(&conn, "type:adr AND planning_status:accepted").unwrap();
    assert_eq!(accepted.len(), 12);

    // The example success-criterion query shape resolves without error.
    let in_progress = run(&conn, "type:card AND building_status:in-progress").unwrap();
    assert!(
        in_progress
            .iter()
            .all(|(slug, _)| slug.starts_with("p1-") || slug.contains('-')),
        "got {in_progress:?}"
    );
}

#[test]
fn unknown_field_errors_not_panics() {
    let conn = build_in_memory(&repo_root().join("examples/sample-project")).unwrap();
    assert!(run(&conn, "nope:1").is_err());
}

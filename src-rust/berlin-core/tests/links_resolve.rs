// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration: the real example and dogfooding projects resolve with zero
//! structural problems — every relation slug points at a real card, slugs are
//! unique, and no waypoint appears in a `criteria` list (ADR-010/011).

use std::path::{Path, PathBuf};

use berlin_core::links::resolve;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn example_project_is_sound() {
    let res = resolve(&repo_root().join("examples/sample-project")).unwrap();
    assert!(res.is_sound(), "problems: {:?}", res.problems);
}

#[test]
fn dogfooding_project_is_sound() {
    let res = resolve(&repo_root().join("berlin-development")).unwrap();
    assert!(
        res.is_sound(),
        "dogfooding project has link problems: {:?}",
        res.problems
    );
    // sanity: it actually loaded the full set, so "sound" isn't vacuous.
    assert!(
        res.by_slug.len() >= 25,
        "loaded {} cards",
        res.by_slug.len()
    );
}

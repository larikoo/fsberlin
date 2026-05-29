// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration: the walker discovers the real example and dogfooding
//! projects, and never surfaces a path inside an opaque directory (ADR-008).

use std::path::{Path, PathBuf};

use berlin_core::walk::discover;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn discovers_example_project() {
    let root = repo_root().join("examples/sample-project");
    let layout = discover(&root).unwrap();
    assert_eq!(layout.cards.len(), 2, "example has two cards");
    assert_eq!(layout.agents.len(), 1, "example has one agent");
    assert!(layout.config.is_some(), "example has a config");
}

#[test]
fn discovers_dogfooding_project_and_skips_opaque() {
    let root = repo_root().join("berlin-development");
    let layout = discover(&root).unwrap();
    assert!(layout.cards.len() >= 20, "dogfooding has the full card set");
    assert_eq!(layout.waypoints.len(), 3, "three dogfooding waypoints");
    assert_eq!(layout.agents.len(), 2, "lari + claude-code");
    assert!(layout.config.is_some());
    for c in &layout.cards {
        let s = c.path.to_string_lossy();
        assert!(
            !s.contains("/.beads/") && !s.contains("/.git/"),
            "leaked an opaque path: {s}"
        );
    }
}

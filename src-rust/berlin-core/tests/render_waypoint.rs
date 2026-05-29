// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration: rendering the example project's waypoint produces base files
//! plus the overlay (ADR-003).

use std::fs;
use std::path::{Path, PathBuf};

use berlin_core::render::render;
use tempfile::tempdir;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn renders_example_waypoint() {
    let root = repo_root().join("examples/sample-project");
    let out = tempdir().unwrap();
    let dest = out.path().join("rendered");

    let report = render(&root, "waypoint-1-brew-day", &dest).unwrap();

    // base files copied
    assert!(dest.join("why.md").is_file());
    assert!(dest.join("cards/card-brew-the-mash/card.md").is_file());
    assert!(dest.join(".fsberlin/config.yaml").is_file());
    // the waypoints/ dir is not part of the projection
    assert!(!dest.join("waypoints").exists());
    // the overlay contributed its file
    assert_eq!(report.overlaid, ["overlay.md"]);
    assert!(fs::read_to_string(dest.join("overlay.md")).is_ok());
    assert!(report.base_files >= 4);
}

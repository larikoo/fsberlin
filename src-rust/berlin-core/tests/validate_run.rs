// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration: the real projects validate clean, and the repo root (not an
//! FSBerlin project) is flagged.

use std::path::{Path, PathBuf};

use berlin_core::validate::validate;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn dogfooding_project_validates_clean() {
    let findings = validate(&repo_root().join("berlin-development")).unwrap();
    assert!(findings.is_empty(), "findings: {findings:?}");
}

#[test]
fn example_project_validates_clean() {
    let findings = validate(&repo_root().join("examples/sample-project")).unwrap();
    assert!(findings.is_empty(), "findings: {findings:?}");
}

#[test]
fn repo_root_is_not_a_project() {
    let findings = validate(&repo_root()).unwrap();
    assert_eq!(findings.len(), 1);
    assert!(findings[0].detail.contains("not an FSBerlin project"));
}

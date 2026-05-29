// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration: every card.md / waypoint.md / agent yaml / project config in
//! the repository parses into its typed model. This is the concrete
//! done-criterion for the p1-frontmatter-parser card.

use std::fs;
use std::path::{Path, PathBuf};

use berlin_core::frontmatter::{parse_agent, parse_card, parse_project};
use berlin_core::model::Card;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

/// `card.md` files directly under each subdirectory of `dir`.
fn card_files_in(dir: &Path, file: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path().join(file);
            if p.is_file() {
                out.push(p);
            }
        }
    }
    out
}

#[test]
fn all_repo_cards_parse() {
    let root = repo_root();
    let mut count = 0;
    for base in [
        root.join("berlin-development/cards"),
        root.join("examples/sample-project/cards"),
    ] {
        for path in card_files_in(&base, "card.md") {
            let text = fs::read_to_string(&path).unwrap();
            parse_card(&text).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()));
            count += 1;
        }
    }
    assert!(
        count >= 20,
        "expected the full card set, parsed only {count}"
    );
}

#[test]
fn all_repo_waypoints_parse_as_waypoints() {
    let root = repo_root();
    let base = root.join("berlin-development/waypoints");
    let mut count = 0;
    for path in card_files_in(&base, "waypoint.md") {
        let text = fs::read_to_string(&path).unwrap();
        let card = parse_card(&text).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()));
        assert!(
            matches!(card, Card::Waypoint(_)),
            "{} should be a waypoint",
            path.display()
        );
        count += 1;
    }
    assert_eq!(count, 3, "expected three dogfooding waypoints");
}

#[test]
fn all_repo_agents_and_configs_parse() {
    let root = repo_root();
    for base in [
        root.join("berlin-development/agents"),
        root.join("examples/sample-project/agents"),
    ] {
        if let Ok(entries) = fs::read_dir(&base) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().is_some_and(|e| e == "yaml") {
                    let text = fs::read_to_string(&p).unwrap();
                    parse_agent(&text).unwrap_or_else(|e| panic!("agent {}: {e}", p.display()));
                }
            }
        }
    }
    for cfg in [
        root.join("berlin-development/.fsberlin/config.yaml"),
        root.join("examples/sample-project/.fsberlin/config.yaml"),
    ] {
        let text = fs::read_to_string(&cfg).unwrap();
        parse_project(&text).unwrap_or_else(|e| panic!("config {}: {e}", cfg.display()));
    }
}

// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The deterministic validators available in Phase 1 (ADR-006): YAML safe-
//! load, schema-shape (typed parse, which also rejects unknown/retired
//! fields), and reference resolution. The full set — state transitions,
//! signed commits, secret scanning — lands in Phase 3.
//!
//! This is the same library code the watcher and the future MCP write path
//! call; there is no validate-only check (ADR-007).

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::Path;

use crate::Result;
use crate::frontmatter::{parse_agent, parse_card, parse_project};
use crate::links;
use crate::model::Card;
use crate::walk;

/// A single validation finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// Where the problem is — a project-relative path, a slug, or a section.
    pub location: String,
    /// What is wrong.
    pub detail: String,
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.location, self.detail)
    }
}

/// Run the Phase 1 validators over the project at `root`.
///
/// Returns every finding; an empty vec means the project is valid. Only
/// unexpected I/O failures are returned as `Err` — validation problems are
/// data, not errors, so the caller can report them all at once.
pub fn validate(root: &Path) -> Result<Vec<Finding>> {
    let mut findings = Vec::new();

    let config_path = root.join(".fsberlin").join("config.yaml");
    if !config_path.is_file() {
        findings.push(Finding {
            location: root.display().to_string(),
            detail: "not an FSBerlin project (no .fsberlin/config.yaml)".into(),
        });
        return Ok(findings);
    }
    if let Err(e) = parse_project(&fs::read_to_string(&config_path)?) {
        findings.push(Finding {
            location: rel(&config_path, root),
            detail: e.to_string(),
        });
    }

    let layout = walk::discover(root)?;

    for agent in &layout.agents {
        if let Err(e) = parse_agent(&fs::read_to_string(&agent.path)?) {
            findings.push(Finding {
                location: rel(&agent.path, root),
                detail: e.to_string(),
            });
        }
    }

    let mut by_slug: BTreeMap<String, Card> = BTreeMap::new();
    let entries = layout
        .cards
        .iter()
        .map(|c| (&c.slug, &c.path))
        .chain(layout.waypoints.iter().map(|w| (&w.slug, &w.path)));
    for (slug, path) in entries {
        match parse_card(&fs::read_to_string(path)?) {
            Ok(card) => {
                if by_slug.insert(slug.clone(), card).is_some() {
                    findings.push(Finding {
                        location: slug.clone(),
                        detail: "duplicate slug (slugs must be project-unique)".into(),
                    });
                }
            }
            Err(e) => findings.push(Finding {
                location: rel(path, root),
                detail: e.to_string(),
            }),
        }
    }

    for problem in links::check(&by_slug) {
        findings.push(Finding {
            location: "relations".into(),
            detail: problem.to_string(),
        });
    }

    Ok(findings)
}

fn rel(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn config(root: &Path) {
        let p = root.join(".fsberlin").join("config.yaml");
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(
            &p,
            "project_id: 01P\nname: Test\ncreated: 2026-05-29\nschema_version: \"0.1\"\n",
        )
        .unwrap();
    }

    fn card(root: &Path, slug: &str, body: &str) {
        let p = root.join("cards").join(slug).join("card.md");
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, format!("---\n{body}\n---\n")).unwrap();
    }

    #[test]
    fn valid_project_has_no_findings() {
        let dir = tempdir().unwrap();
        config(dir.path());
        card(
            dir.path(),
            "a",
            "id: 01a\ntitle: A\ntype: card\nbuilding_status: pending\ncreated: 2026-05-29\ndepends_on: [b]",
        );
        card(
            dir.path(),
            "b",
            "id: 01b\ntitle: B\ntype: card\nbuilding_status: done\ncreated: 2026-05-29",
        );
        let findings = validate(dir.path()).unwrap();
        assert!(findings.is_empty(), "findings: {findings:?}");
    }

    #[test]
    fn non_project_is_flagged() {
        let dir = tempdir().unwrap();
        let findings = validate(dir.path()).unwrap();
        assert_eq!(findings.len(), 1);
        assert!(findings[0].detail.contains("not an FSBerlin project"));
    }

    #[test]
    fn malformed_card_is_located() {
        let dir = tempdir().unwrap();
        config(dir.path());
        card(dir.path(), "bad", "id: [unterminated");
        let findings = validate(dir.path()).unwrap();
        assert!(
            findings.iter().any(|f| f.location.contains("bad")),
            "findings: {findings:?}"
        );
    }

    #[test]
    fn retired_status_field_is_flagged() {
        let dir = tempdir().unwrap();
        config(dir.path());
        card(
            dir.path(),
            "legacy",
            "id: 01l\ntitle: L\ntype: card\nstatus: pending\ncreated: 2026-05-29",
        );
        let findings = validate(dir.path()).unwrap();
        assert!(
            findings.iter().any(|f| f.detail.contains("status")),
            "findings: {findings:?}"
        );
    }

    #[test]
    fn dangling_reference_is_flagged() {
        let dir = tempdir().unwrap();
        config(dir.path());
        card(
            dir.path(),
            "a",
            "id: 01a\ntitle: A\ntype: card\nbuilding_status: pending\ncreated: 2026-05-29\ndepends_on: [ghost]",
        );
        let findings = validate(dir.path()).unwrap();
        assert!(
            findings
                .iter()
                .any(|f| f.location == "relations" && f.detail.contains("ghost")),
            "findings: {findings:?}"
        );
    }
}

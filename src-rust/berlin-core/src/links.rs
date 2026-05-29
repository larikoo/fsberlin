// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolving cross-card relations by slug (ADR-010) over a walked + parsed
//! project, and reporting the structural problems a deterministic validator
//! must block on: dangling references, duplicate slugs, and the ADR-011
//! acyclicity guardrail (a waypoint slug must never appear in a `criteria`
//! list).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use crate::Result;
use crate::frontmatter::parse_card;
use crate::model::{Card, CardType};
use crate::walk;

/// Which relation field a reference came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationField {
    DependsOn,
    Blocks,
    Linked,
    Criteria,
}

impl fmt::Display for RelationField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RelationField::DependsOn => "depends_on",
            RelationField::Blocks => "blocks",
            RelationField::Linked => "linked",
            RelationField::Criteria => "criteria",
        };
        f.write_str(s)
    }
}

/// A structural problem found while resolving relations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkProblem {
    /// A relation references a slug that resolves to no card.
    Dangling {
        from: String,
        field: RelationField,
        to: String,
    },
    /// Two cards share a slug (slugs are project-unique, ADR-010).
    DuplicateSlug { slug: String },
    /// A waypoint slug appears in a `criteria` list (forbidden, ADR-011).
    WaypointInCriteria { owner: String, waypoint: String },
}

impl fmt::Display for LinkProblem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkProblem::Dangling { from, field, to } => {
                write!(f, "{from}: {field} -> `{to}` does not resolve")
            }
            LinkProblem::DuplicateSlug { slug } => {
                write!(f, "duplicate slug `{slug}` (slugs must be project-unique)")
            }
            LinkProblem::WaypointInCriteria { owner, waypoint } => write!(
                f,
                "{owner}: criteria references waypoint `{waypoint}` (waypoints may not be criteria — ADR-011)"
            ),
        }
    }
}

/// The result of resolving a project's relations.
#[derive(Debug, Default)]
pub struct Resolution {
    /// Every card, keyed by its (project-unique) slug.
    pub by_slug: BTreeMap<String, Card>,
    /// Structural problems found; empty means the graph is sound.
    pub problems: Vec<LinkProblem>,
}

impl Resolution {
    /// Whether the relation graph is sound (no problems).
    #[must_use]
    pub fn is_sound(&self) -> bool {
        self.problems.is_empty()
    }
}

/// Walk, parse, and resolve the relations of the project rooted at `root`.
///
/// Parse/I-O failures are returned as `Err`; *structural* problems (dangling
/// refs, duplicate slugs, waypoint-in-criteria) are collected in
/// [`Resolution::problems`] so a caller can report them all at once.
pub fn resolve(root: &Path) -> Result<Resolution> {
    let layout = walk::discover(root)?;
    let mut by_slug: BTreeMap<String, Card> = BTreeMap::new();
    let mut problems = Vec::new();

    let entries = layout
        .cards
        .iter()
        .map(|c| (&c.slug, &c.path))
        .chain(layout.waypoints.iter().map(|w| (&w.slug, &w.path)));
    for (slug, path) in entries {
        let text = fs::read_to_string(path)?;
        let card = parse_card(&text)?;
        if by_slug.insert(slug.clone(), card).is_some() {
            problems.push(LinkProblem::DuplicateSlug { slug: slug.clone() });
        }
    }

    let waypoint_slugs: BTreeSet<&str> = by_slug
        .iter()
        .filter(|(_, c)| c.card_type() == CardType::Waypoint)
        .map(|(s, _)| s.as_str())
        .collect();

    for (slug, card) in &by_slug {
        for to in card.depends_on() {
            check_resolves(slug, RelationField::DependsOn, to, &by_slug, &mut problems);
        }
        for to in card.blocks() {
            check_resolves(slug, RelationField::Blocks, to, &by_slug, &mut problems);
        }
        for to in card.linked() {
            check_resolves(slug, RelationField::Linked, to, &by_slug, &mut problems);
        }
        for to in card.criteria() {
            if !by_slug.contains_key(to) {
                problems.push(LinkProblem::Dangling {
                    from: slug.clone(),
                    field: RelationField::Criteria,
                    to: to.clone(),
                });
            } else if waypoint_slugs.contains(to.as_str()) {
                problems.push(LinkProblem::WaypointInCriteria {
                    owner: slug.clone(),
                    waypoint: to.clone(),
                });
            }
        }
    }

    Ok(Resolution { by_slug, problems })
}

fn check_resolves(
    from: &str,
    field: RelationField,
    to: &str,
    by_slug: &BTreeMap<String, Card>,
    problems: &mut Vec<LinkProblem>,
) {
    if !by_slug.contains_key(to) {
        problems.push(LinkProblem::Dangling {
            from: from.to_string(),
            field,
            to: to.to_string(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn write(root: &Path, kind: &str, slug: &str, body: &str) {
        let (dir, file) = match kind {
            "waypoint" => ("waypoints", "waypoint.md"),
            _ => ("cards", "card.md"),
        };
        let p = root.join(dir).join(slug).join(file);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, format!("---\n{body}\n---\n")).unwrap();
    }

    fn work(slug: &str, rel: &str) -> (String, String, String) {
        (
            "card".into(),
            slug.into(),
            format!(
                "id: 01{slug}\ntitle: {slug}\ntype: card\nbuilding_status: pending\ncreated: 2026-05-29\n{rel}"
            ),
        )
    }

    #[test]
    fn sound_graph_has_no_problems() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        write(
            root,
            "card",
            "a",
            "id: 01a\ntitle: A\ntype: card\nbuilding_status: pending\ncreated: 2026-05-29\ndepends_on: [b]",
        );
        write(
            root,
            "card",
            "b",
            "id: 01b\ntitle: B\ntype: card\nbuilding_status: pending\ncreated: 2026-05-29\nblocks: [a]",
        );
        let res = resolve(root).unwrap();
        assert!(res.is_sound(), "problems: {:?}", res.problems);
        assert_eq!(res.by_slug.len(), 2);
    }

    #[test]
    fn dangling_reference_is_reported() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let (_, _, body) = work("a", "depends_on: [missing]");
        write(root, "card", "a", &body);
        let res = resolve(root).unwrap();
        assert_eq!(res.problems.len(), 1);
        assert!(matches!(
            &res.problems[0],
            LinkProblem::Dangling { to, field: RelationField::DependsOn, .. } if to == "missing"
        ));
    }

    #[test]
    fn waypoint_in_criteria_is_rejected() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        write(
            root,
            "waypoint",
            "wp-x",
            "id: 01wp\nslug: wp-x\ntitle: WP\ntype: waypoint\nstatus: active\ncreated: 2026-05-29\ncriteria: []",
        );
        write(
            root,
            "card",
            "ph",
            "id: 01ph\ntitle: Ph\ntype: phase\nphase_number: 9\ncriteria: [wp-x]\ncreated: 2026-05-29",
        );
        let res = resolve(root).unwrap();
        assert_eq!(res.problems.len(), 1, "problems: {:?}", res.problems);
        assert!(matches!(
            &res.problems[0],
            LinkProblem::WaypointInCriteria { waypoint, .. } if waypoint == "wp-x"
        ));
    }

    #[test]
    fn duplicate_slug_across_cards_and_waypoints_is_reported() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        write(
            root,
            "card",
            "dup",
            "id: 01c\ntitle: C\ntype: card\nbuilding_status: pending\ncreated: 2026-05-29",
        );
        write(
            root,
            "waypoint",
            "dup",
            "id: 01w\nslug: dup\ntitle: W\ntype: waypoint\nstatus: active\ncreated: 2026-05-29\ncriteria: []",
        );
        let res = resolve(root).unwrap();
        assert!(
            res.problems
                .iter()
                .any(|p| matches!(p, LinkProblem::DuplicateSlug { slug } if slug == "dup")),
            "problems: {:?}",
            res.problems
        );
    }
}

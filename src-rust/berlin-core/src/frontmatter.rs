// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Extracting and parsing YAML frontmatter into the typed models in
//! [`crate::model`]. Parsing is serde_yaml safe-load only (no arbitrary
//! tags), per SPEC §4 and ADR-007.

use serde::Deserialize;

use crate::model::{AdrCard, Agent, Card, CardType, PhaseCard, Project, WaypointCard, WorkCard};
use crate::{Error, Result};

/// Return the YAML frontmatter at the head of a markdown file: the text
/// between the opening `---` line and the next `---` line.
///
/// ADR cards keep prose (and a `---` rule) in their body; only the *first*
/// fenced block is treated as frontmatter.
pub fn extract_frontmatter(text: &str) -> Result<&str> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let after_open = text
        .strip_prefix("---\n")
        .or_else(|| text.strip_prefix("---\r\n"))
        .ok_or(Error::MissingFrontmatter)?;
    let end = after_open.find("\n---").ok_or(Error::MissingFrontmatter)?;
    Ok(&after_open[..end])
}

#[derive(Deserialize)]
struct TypeProbe {
    #[serde(rename = "type")]
    card_type: CardType,
}

/// Parse a `card.md` or `waypoint.md` into a typed [`Card`].
///
/// Dispatches on `type:`. Unknown or retired fields (e.g. the retired
/// universal `status:`) are rejected by the per-type models.
pub fn parse_card(md: &str) -> Result<Card> {
    let fm = extract_frontmatter(md)?;
    let value: serde_yaml::Value = serde_yaml::from_str(fm)?;
    let probe: TypeProbe = serde_yaml::from_value(value.clone())?;
    let card = match probe.card_type {
        CardType::Card => Card::Work(serde_yaml::from_value::<WorkCard>(value)?),
        CardType::Adr => Card::Adr(serde_yaml::from_value::<AdrCard>(value)?),
        CardType::Phase => Card::Phase(serde_yaml::from_value::<PhaseCard>(value)?),
        CardType::Waypoint => Card::Waypoint(serde_yaml::from_value::<WaypointCard>(value)?),
    };
    Ok(card)
}

/// Parse an agent definition file (`agents/<id>.yaml`). The whole file is
/// YAML; leading comments are ignored.
pub fn parse_agent(yaml: &str) -> Result<Agent> {
    Ok(serde_yaml::from_str(yaml)?)
}

/// Parse a project config file (`.fsberlin/config.yaml`).
pub fn parse_project(yaml: &str) -> Result<Project> {
    Ok(serde_yaml::from_str(yaml)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{BuildingStatus, PlanningStatus, WaypointStatus};

    #[test]
    fn parses_work_card() {
        let md = "---\nid: 01AAA\ntitle: Do the thing\ntype: card\nbuilding_status: in-progress\ndepends_on: [other-card]\nphase: 1\ncreated: 2026-05-29\n---\n\nbody\n";
        let Card::Work(c) = parse_card(md).unwrap() else {
            panic!("expected work card");
        };
        assert_eq!(c.building_status, BuildingStatus::InProgress);
        assert_eq!(c.depends_on, vec!["other-card"]);
        assert_eq!(c.phase, Some(1));
    }

    #[test]
    fn parses_adr_card_with_clause_supersession() {
        let md = "---\nid: 01ADR\ntitle: A decision\ntype: adr\nadr_number: 10\nplanning_status: accepted\nsupersedes: [adr-002§002]\ncreated: 2026-05-29\n---\n";
        let Card::Adr(c) = parse_card(md).unwrap() else {
            panic!("expected adr card");
        };
        assert_eq!(c.adr_number, 10);
        assert_eq!(c.planning_status, PlanningStatus::Accepted);
        assert_eq!(c.supersedes, vec!["adr-002§002"]);
    }

    #[test]
    fn parses_phase_card() {
        let md = "---\nid: 01PH\ntitle: A phase\ntype: phase\nphase_number: 1\ncriteria:\n  - card-a\n  - card-b\ncreated: 2026-05-29\n---\n";
        let Card::Phase(c) = parse_card(md).unwrap() else {
            panic!("expected phase card");
        };
        assert_eq!(c.phase_number, 1);
        assert_eq!(c.criteria, vec!["card-a", "card-b"]);
    }

    #[test]
    fn parses_waypoint_card() {
        let md = "---\nid: 01WP\nslug: wp-1\ntitle: A milestone\ntype: waypoint\nstatus: active\nreached_date: 2026-05-29\ncriteria: [phase-0-spec-and-schema]\ncreated: 2026-05-25\n---\n";
        let Card::Waypoint(c) = parse_card(md).unwrap() else {
            panic!("expected waypoint card");
        };
        assert_eq!(c.status, WaypointStatus::Active);
        assert_eq!(c.slug, "wp-1");
        assert_eq!(c.criteria, vec!["phase-0-spec-and-schema"]);
    }

    #[test]
    fn rejects_retired_status_field() {
        let md =
            "---\nid: 01X\ntitle: Legacy\ntype: card\nstatus: pending\ncreated: 2026-05-29\n---\n";
        let err = parse_card(md).unwrap_err();
        // deny_unknown_fields surfaces the retired field by name.
        assert!(err.to_string().contains("status"), "got: {err}");
    }

    #[test]
    fn errors_on_missing_frontmatter() {
        assert!(matches!(
            parse_card("no fences here\n").unwrap_err(),
            Error::MissingFrontmatter
        ));
    }

    #[test]
    fn errors_on_malformed_yaml_without_panic() {
        let md = "---\nid: [unterminated\n---\n";
        assert!(matches!(parse_card(md).unwrap_err(), Error::Yaml(_)));
    }

    #[test]
    fn parses_agent() {
        let yaml = "# comment\nid: spy\ntype: ai\nrole: checker\nwhy: catches conflicts\ndefault_model: m\npermitted_models: [m]\nread_scope: [\"**/*\"]\nwrite_scope: [findings/]\nsandbox: true\n";
        let a = parse_agent(yaml).unwrap();
        assert_eq!(a.id, "spy");
        assert_eq!(a.sandbox, Some(true));
    }

    #[test]
    fn parses_project() {
        let yaml = "project_id: 01P\nname: Demo\ncreated: 2026-05-25\nschema_version: \"0.1\"\nopaque_paths: [.git, .beads]\n";
        let p = parse_project(yaml).unwrap();
        assert_eq!(p.schema_version, "0.1");
        assert_eq!(p.opaque_paths, vec![".git", ".beads"]);
    }

    #[test]
    fn extract_takes_only_the_first_fenced_block() {
        let md = "---\ntype: adr\n---\n\nbody with a rule\n\n---\n\nmore body\n";
        assert_eq!(extract_frontmatter(md).unwrap(), "type: adr");
    }

    #[test]
    fn work_card_round_trips() {
        let md = "---\nid: 01RT\ntitle: Round trip\ntype: card\nbuilding_status: done\ncreated: 2026-05-29\n---\n";
        let Card::Work(c) = parse_card(md).unwrap() else {
            panic!("expected work card");
        };
        let yaml = serde_yaml::to_string(&c).unwrap();
        let back: WorkCard = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(c, back);
    }
}

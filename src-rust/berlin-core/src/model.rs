// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Typed models for FSBerlin frontmatter, mirroring the locked schemas in
//! `schema/`. Card status fields are type-specialized per ADR-009; waypoint
//! fields follow ADR-011.
//!
//! Each card struct lists its fields explicitly and uses
//! `deny_unknown_fields` (rather than `#[serde(flatten)]` of a shared struct)
//! for two reasons: it rejects unknown/retired fields such as the retired
//! universal `status:`, and it avoids a serde_yaml flatten bug that
//! mis-deserializes flattened numeric fields like `phase`.

use serde::{Deserialize, Serialize};

/// The `type:` discriminator carried by every card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardType {
    Card,
    Adr,
    Phase,
    Waypoint,
}

/// Execution lifecycle for work cards (`type: card`), per ADR-009.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuildingStatus {
    Pending,
    InProgress,
    Review,
    Done,
    Blocked,
    Archived,
}

/// Ratification lifecycle for ADR cards, per ADR-009.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlanningStatus {
    Proposed,
    InDiscussion,
    Accepted,
    Superseded,
    Withdrawn,
}

/// Human-intent lifecycle for waypoints, per ADR-011. "Reached" is derived,
/// never stored, so it is deliberately absent from this enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WaypointStatus {
    Planned,
    Active,
    Abandoned,
}

/// Priority, shared across card types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl CardType {
    /// The lowercase string form used in frontmatter and the index.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            CardType::Card => "card",
            CardType::Adr => "adr",
            CardType::Phase => "phase",
            CardType::Waypoint => "waypoint",
        }
    }
}

impl BuildingStatus {
    /// The frontmatter string form (kebab-case).
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            BuildingStatus::Pending => "pending",
            BuildingStatus::InProgress => "in-progress",
            BuildingStatus::Review => "review",
            BuildingStatus::Done => "done",
            BuildingStatus::Blocked => "blocked",
            BuildingStatus::Archived => "archived",
        }
    }
}

impl PlanningStatus {
    /// The frontmatter string form (kebab-case).
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            PlanningStatus::Proposed => "proposed",
            PlanningStatus::InDiscussion => "in-discussion",
            PlanningStatus::Accepted => "accepted",
            PlanningStatus::Superseded => "superseded",
            PlanningStatus::Withdrawn => "withdrawn",
        }
    }
}

impl WaypointStatus {
    /// The frontmatter string form (lowercase).
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            WaypointStatus::Planned => "planned",
            WaypointStatus::Active => "active",
            WaypointStatus::Abandoned => "abandoned",
        }
    }
}

impl Priority {
    /// The frontmatter string form (lowercase).
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
            Priority::Critical => "critical",
        }
    }
}

/// A work card (`type: card`): execution lifecycle only (ADR-009).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkCard {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub card_type: CardType,
    pub created: String,
    pub building_status: BuildingStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub waypoints: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// An ADR card (`type: adr`): ratification lifecycle (ADR-009). Supports
/// clause-granular supersession references (`adr-002§002`) per ADR-012.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdrCard {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub card_type: CardType,
    pub created: String,
    pub adr_number: u32,
    pub planning_status: PlanningStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supersedes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub superseded_by: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub waypoints: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// A phase card (`type: phase`): a gate over a `criteria` list (ADR-009). The
/// "met" state is derived from the criteria cards, never stored here.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhaseCard {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub card_type: CardType,
    pub created: String,
    pub phase_number: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub criteria: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_weeks: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub waypoints: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// A waypoint card (`type: waypoint`): milestone projection (ADR-003/011).
/// `status` is human intent; whether it is reached is derived from `criteria`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WaypointCard {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub card_type: CardType,
    pub created: String,
    pub slug: String,
    pub status: WaypointStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub criteria: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reached_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// A parsed card of any type.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Card {
    Work(WorkCard),
    Adr(AdrCard),
    Phase(PhaseCard),
    Waypoint(WaypointCard),
}

impl Card {
    /// The stable ULID `id` of this card.
    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            Card::Work(c) => &c.id,
            Card::Adr(c) => &c.id,
            Card::Phase(c) => &c.id,
            Card::Waypoint(c) => &c.id,
        }
    }

    /// The human title of this card.
    #[must_use]
    pub fn title(&self) -> &str {
        match self {
            Card::Work(c) => &c.title,
            Card::Adr(c) => &c.title,
            Card::Phase(c) => &c.title,
            Card::Waypoint(c) => &c.title,
        }
    }

    /// The `type:` of this card.
    #[must_use]
    pub fn card_type(&self) -> CardType {
        match self {
            Card::Work(c) => c.card_type,
            Card::Adr(c) => c.card_type,
            Card::Phase(c) => c.card_type,
            Card::Waypoint(c) => c.card_type,
        }
    }

    /// Cards this card depends on (slugs, ADR-010).
    #[must_use]
    pub fn depends_on(&self) -> &[String] {
        match self {
            Card::Work(c) => &c.depends_on,
            Card::Adr(c) => &c.depends_on,
            Card::Phase(c) => &c.depends_on,
            Card::Waypoint(c) => &c.depends_on,
        }
    }

    /// Cards blocked by this card (slugs).
    #[must_use]
    pub fn blocks(&self) -> &[String] {
        match self {
            Card::Work(c) => &c.blocks,
            Card::Adr(c) => &c.blocks,
            Card::Phase(c) => &c.blocks,
            Card::Waypoint(c) => &c.blocks,
        }
    }

    /// Related-but-not-blocking cards (slugs).
    #[must_use]
    pub fn linked(&self) -> &[String] {
        match self {
            Card::Work(c) => &c.linked,
            Card::Adr(c) => &c.linked,
            Card::Phase(c) => &c.linked,
            Card::Waypoint(c) => &c.linked,
        }
    }

    /// Criteria slugs (phase and waypoint cards only; empty otherwise).
    #[must_use]
    pub fn criteria(&self) -> &[String] {
        match self {
            Card::Phase(c) => &c.criteria,
            Card::Waypoint(c) => &c.criteria,
            _ => &[],
        }
    }

    /// The ISO-8601 creation date.
    #[must_use]
    pub fn created(&self) -> &str {
        match self {
            Card::Work(c) => &c.created,
            Card::Adr(c) => &c.created,
            Card::Phase(c) => &c.created,
            Card::Waypoint(c) => &c.created,
        }
    }

    /// The card's priority, if set.
    #[must_use]
    pub fn priority(&self) -> Option<Priority> {
        match self {
            Card::Work(c) => c.priority,
            Card::Adr(c) => c.priority,
            Card::Phase(c) => c.priority,
            Card::Waypoint(c) => c.priority,
        }
    }

    /// The phase number this card belongs to, if set.
    #[must_use]
    pub fn phase(&self) -> Option<u32> {
        match self {
            Card::Work(c) => c.phase,
            Card::Adr(c) => c.phase,
            Card::Phase(c) => c.phase,
            Card::Waypoint(c) => c.phase,
        }
    }
}

/// Agent kind (ADR-004).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentType {
    Human,
    Ai,
}

/// An agent definition (`agents/<id>.yaml`), per ADR-004.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Agent {
    pub id: String,
    #[serde(rename = "type")]
    pub agent_type: AgentType,
    pub role: String,
    pub why: String,
    pub default_model: String,
    pub permitted_models: Vec<String>,
    pub read_scope: Vec<String>,
    pub write_scope: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_access: Option<bool>,
}

/// Project config (`.fsberlin/config.yaml`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub project_id: String,
    pub name: String,
    pub created: String,
    pub schema_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stewards: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub opaque_paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validators_config: Option<serde_yaml::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watcher_config: Option<serde_yaml::Value>,
}

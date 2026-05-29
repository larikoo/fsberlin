---
id: 01JBP1PARSER00000000000000
title: "Typed frontmatter parser (serde_yaml, safe-load)"
type: card
building_status: done
priority: high
phase: 1
assignee: claude-code
skills: [rust]
depends_on: [p1-workspace-scaffold, adr-009-two-dimensional-status]
created: 2026-05-29
---

Parse card/agent/waypoint/project frontmatter into typed Rust structs that
mirror the locked schemas.

## Deliverable
- serde + serde_yaml parsing, **safe-load only** (no arbitrary tags).
- Type-specialized card models: work (`building_status`), ADR
  (`planning_status`, `adr_number`), phase (`criteria`, `phase_number`),
  waypoint (`status`, `criteria`) — per ADR-009 and ADR-011.
- Round-trips: parse → struct → serialize preserves human-writeable form
  (one field per line; ADR-007).

## Done when
- Every card.md, agent yaml, and waypoint.md in the repo parses into its
  typed model.
- Malformed YAML yields a clear, located error (no panic).
- Unknown/legacy fields (e.g. retired `status:`) are rejected or flagged.

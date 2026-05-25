---
id: 01JBWORK010000000000000000
title: "Lock the four schema files"
type: card
building_status: in-progress
priority: high
phase: 0
assignee: claude-code
skills: [architecture, design]
depends_on: [adr-009-two-dimensional-status]
created: 2026-05-25
---

Finalize and lock `schema/card.schema.yaml`, `schema/agent.schema.yaml`,
`schema/project.schema.yaml`, and `schema/waypoint.schema.yaml`.

Each schema file should be internally consistent, reference any governing
ADRs, and describe all currently-used fields. The card schema has been
updated per ADR-009. The remaining three need review and any necessary
updates before Phase 0 can close.

## Done when
- All four schema files are consistent with the accepted ADRs.
- No field in any existing card is undocumented in the schema.
- Each schema file has a clear status comment (draft → locked).

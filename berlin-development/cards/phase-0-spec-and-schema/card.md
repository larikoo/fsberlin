---
id: 01JBPHASE0000000000000000
title: "SPEC.md and schemas locked"
type: phase
phase_number: 0
criteria:
  - adr-001-filesystem-as-substrate
  - adr-002-card-equals-folder
  - adr-003-waypoints-as-overlays
  - adr-004-agents-vs-models
  - adr-005-hitl-floors
  - adr-006-validators-vs-spymaster
  - adr-007-editor-as-peer
  - adr-008-stacks-not-absorbs
  - adr-009-two-dimensional-status
  - adr-010-relation-identity
  - lock-four-schemas
  - spec-fill-placeholders
  - why-md-review
priority: high
assignee: claude-code
skills: [rust, python, architecture]
depends_on: []
created: 2026-05-25
---

Phase 0 of FSBerlin development.


Lock the four schema files (card, agent, project, waypoint). Promote
all eight ADRs from `draft` to `accepted`. Finish SPEC.md so no
section says "to be expanded."

## Success criteria
- All nine ADRs have `planning_status: accepted`.
- Schema files validate against themselves (self-describing where
  possible).
- SPEC.md has no `(To be expanded ...)` markers.
- `docs/why.md` reads cleanly to someone unfamiliar with the project.

## Out of scope
- Implementation code (that's Phase 1+).
- View renderer specifics (that's Phase 9).

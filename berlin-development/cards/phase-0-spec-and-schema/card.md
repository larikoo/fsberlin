---
id: 01JBPHASE0000000000000000
title: "SPEC.md and schemas locked"
type: phase
phase_number: 0
status: pending
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
- All eight ADRs have `status: accepted`.
- Schema files validate against themselves (self-describing where
  possible).
- SPEC.md has no `(To be expanded ...)` markers.
- `docs/why.md` reads cleanly to someone unfamiliar with the project.

## Out of scope
- Implementation code (that's Phase 1+).
- View renderer specifics (that's Phase 9).

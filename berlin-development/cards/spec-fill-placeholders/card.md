---
id: 01JBWORK020000000000000000
title: "Fill SPEC.md placeholder sections"
type: card
building_status: review
priority: high
phase: 0
assignee: claude-code
skills: [architecture, design]
depends_on: [adr-009-two-dimensional-status]
created: 2026-05-25
---

Remove all `(To be expanded ...)` markers from `docs/SPEC.md` and
replace them with complete specification text.

Currently 5 markers remain:
- Card schema section
- MCP tool surface and CLI verbs
- Validator detail (refs ADR-006, Phase 3)
- Authority / HITL detail (refs ADR-004, ADR-005, Phase 5) — two markers

The Phase 3 and Phase 5 sections can be brief but must describe the
intended behaviour clearly enough to implement against. They are
not deferred — they are Phase 0 spec work.

## Done when
- `grep -r "To be expanded" docs/SPEC.md` returns nothing.
- Each formerly-placeholder section is complete enough to implement
  against without further spec work.

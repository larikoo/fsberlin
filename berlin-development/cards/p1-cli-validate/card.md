---
id: 01JBP1CLIVALID000000000000
title: "berlin validate: schema + reference checks (basic)"
type: card
building_status: pending
priority: high
phase: 1
assignee: claude-code
skills: [rust]
depends_on: [p1-link-resolution, adr-006-validators-vs-spymaster]
created: 2026-05-29
---

`berlin validate [path]` runs the deterministic checks available in Phase 1.
The full validator set (state transitions, signing, secrets) lands in Phase 3
(ADR-006); this is the mechanical subset.

## Deliverable
- YAML safe-load, schema validation against `schema/*.schema.yaml`, and
  reference resolution (dangling slugs, slug uniqueness, acyclicity).
- Exit non-zero on any finding; print located, actionable messages.
- Same library code the watcher/parser use — no validate-only path (ADR-007).

## Done when
- `berlin validate` passes clean on the dogfooding project and the example.
- Fixtures with a bad schema field, a dangling slug, and a duplicate slug
  each fail with a precise message.

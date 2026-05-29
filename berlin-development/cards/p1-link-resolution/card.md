---
id: 01JBP1LINKRES0000000000000
title: "ULID identity + slug relation resolution"
type: card
building_status: pending
priority: high
phase: 1
assignee: claude-code
skills: [rust]
depends_on: [p1-fs-walker, p1-frontmatter-parser, adr-010-relation-identity]
created: 2026-05-29
---

Resolve cross-card relations by slug (ADR-010) over the walked + parsed
project, keying internal identity on the ULID `id`.

## Deliverable
- Build a slug → card index; resolve `depends_on` / `blocks` / `linked` /
  `criteria` entries to cards.
- Detect dangling references (slug with no card) and report card + field.
- Enforce slug uniqueness within a project.
- Detect cycles where they are illegal (e.g. a waypoint slug appearing in any
  `criteria` list — ADR-011 acyclicity guardrail).

## Done when
- Resolving the dogfooding project yields zero dangling refs.
- A fixture with a bad slug and one with a duplicate slug each produce a
  precise error.
- A fixture placing a waypoint in a `criteria` list is rejected.

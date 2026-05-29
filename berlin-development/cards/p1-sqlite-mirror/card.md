---
id: 01JBP1SQLITE00000000000000
title: "SQLite index mirror (regenerable)"
type: card
building_status: pending
priority: high
phase: 1
assignee: claude-code
skills: [rust]
depends_on: [p1-fs-walker, p1-frontmatter-parser, adr-002-card-equals-folder]
created: 2026-05-29
---

Mirror parsed frontmatter into `.fsberlin/index.sqlite` for fast queries. The
index is a cache, never a store (ADR-001, ADR-002).

## Deliverable
- rusqlite-backed schema mirroring card fields + relations.
- Full rebuild from the filesystem (walk → parse → resolve → insert).
- Deterministic: same FS state yields the same index.

## Done when
- Building the index over the example project populates expected rows.
- Deleting `index.sqlite` and rebuilding reproduces identical contents.
- The index is never treated as authoritative — a divergence test proves the
  filesystem wins on rebuild.

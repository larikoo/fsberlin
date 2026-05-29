---
id: 01JBP1CLIQUERY000000000000
title: "berlin query: query the index"
type: card
building_status: done
priority: high
phase: 1
assignee: claude-code
skills: [rust]
depends_on: [p1-sqlite-mirror]
created: 2026-05-29
---

`berlin query <expr>` runs structured queries against the SQLite mirror.

## Deliverable
- Minimal query grammar: field predicates joined by `AND` / `OR`
  (e.g. `type:card AND building_status:in-progress`).
- Resolves against the index; prints matching cards (slug + title).

## Done when
- `berlin query "type:card AND building_status:in-progress"` returns the
  expected card(s) from the example project.
- An unparseable expression yields a clear error, not a panic.

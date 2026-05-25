---
id: 01JBPHASE0100000000000000
title: "Substrate core: FS, parser, index, watcher"
type: phase
phase_number: 1
status: blocked
priority: high
assignee: claude-code
skills: [rust, python, architecture]
depends_on: []
created: 2026-05-25
---

Phase 1 of FSBerlin development.


Implement the Rust substrate core. Single-user, no auth, no MCP yet.

## Deliverables
- `berlin-core` crate: FS layout walker, frontmatter parser (serde +
  serde_yaml), UUID handling, link resolution.
- SQLite mirror via rusqlite. Schema regenerable from filesystem.
- File watcher via notify crate. Debounce 200ms. Atomic-rename aware.
- `berlin-cli` crate with: `init`, `validate`, `query`, `watch`,
  `render-waypoint`.

## Success criteria
- `berlin init my-project/` creates a valid project on disk.
- Editing a card in vim triggers the watcher within ~250ms.
- Watcher absorbs vim/VS Code save bursts without firing partial
  parses.
- SQLite mirror reflects filesystem state and is regenerable by
  deleting the cache file and restarting.
- `berlin query "type:card AND status:in-progress"` returns matching
  cards.

## Depends on
- Phase 0 complete.

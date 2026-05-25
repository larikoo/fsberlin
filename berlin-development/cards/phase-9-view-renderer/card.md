---
id: 01JBPHASE0900000000000000
title: "Minimal read-only web UI (optional, post-MVP)"
type: phase
phase_number: 9
criteria: []
priority: high
assignee: claude-code
skills: [rust, python, architecture]
depends_on: []
created: 2026-05-25
---

Phase 9 of FSBerlin development.


OPTIONAL post-MVP. Minimal read-only web UI.

## Deliverables
- Read-only web view: kanban (by status), tree (by waypoint), card
  detail (full body with frontmatter), waypoint diff (overlay vs
  base).
- No write actions; UI is observational.
- Single static binary or container.
- Mobile-OK (responsive), not mobile-first.

## Success criteria
- Browser at localhost shows the project state.
- Reflects filesystem state with watcher-driven updates.

## Depends on
- Phase 4 (reads via MCP).

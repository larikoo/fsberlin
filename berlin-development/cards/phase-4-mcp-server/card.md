---
id: 01JBPHASE0400000000000000
title: "MCP server exposing ~10 tools (rmcp, Rust)"
type: phase
phase_number: 4
criteria: []
priority: high
assignee: claude-code
skills: [rust, python, architecture]
depends_on: []
created: 2026-05-25
---

Phase 4 of FSBerlin development.


Implement the MCP server (Rust, rmcp crate).

## Deliverables
- `berlin-mcp` binary exposing tools:
  - `list_cards`, `get_card`, `create_card`, `update_card`
  - `link_cards`, `query`, `render_waypoint`, `snapshot_waypoint`
  - `promote_memo`, `sign_approval`
- Stdio transport (default). SSE transport (opt-in).
- Reads via SQLite mirror; writes through validator path.
- Last-known-SHA conflict detection.

## Success criteria
- Claude Code can drive FSBerlin via stdio MCP.
- All ten tools have JSON Schema-described inputs/outputs.
- SSE transport works over localhost with token auth.

## Depends on
- Phases 1, 2, 3.

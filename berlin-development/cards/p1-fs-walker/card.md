---
id: 01JBP1WALKER00000000000000
title: "Project layout walker (honors opaque_paths)"
type: card
building_status: done
priority: high
phase: 1
assignee: claude-code
skills: [rust]
depends_on: [p1-workspace-scaffold, adr-008-stacks-not-absorbs]
created: 2026-05-29
---

Walk a project tree and classify its contents per the SPEC §2.1 layout.

## Deliverable
- Walker that discovers `cards/`, `agents/`, `waypoints/`, `.fsberlin/`,
  `findings/` and yields typed entries.
- Respects `opaque_paths` (ADR-008): never descends into `.git`, `.beads`,
  etc. Defaults from project.schema; configurable.
- Returns structured results, not raw paths.

## Done when
- Given the example project, the walker enumerates exactly its cards/agents/
  waypoints and skips every opaque dir.
- A test fixture with a `.beads/` dir proves it is never read.

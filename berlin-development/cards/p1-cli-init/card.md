---
id: 01JBP1CLIINIT0000000000000
title: "berlin init: scaffold a valid project"
type: card
building_status: done
priority: high
phase: 1
assignee: claude-code
skills: [rust]
depends_on: [p1-fs-walker]
created: 2026-05-29
---

`berlin init <path>` creates a new, valid FSBerlin project on disk per the
SPEC §2.1 layout.

## Deliverable
- Creates `why.md`, `.fsberlin/config.yaml` (valid per project.schema),
  `cards/`, `agents/`, `waypoints/`, `findings/spymaster|sentinel/`.
- Generates a project ULID and `schema_version`.
- Idempotent / refuses to clobber a non-empty target.

## Done when
- `berlin init my-project/` produces a tree the walker + parser accept with
  no errors.
- Re-running on an existing project is safe (no clobber).

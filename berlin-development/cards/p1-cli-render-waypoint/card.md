---
id: 01JBP1CLIRENDER00000000000
title: "berlin render-waypoint: base + overlay projection"
type: card
building_status: done
priority: medium
phase: 1
assignee: claude-code
skills: [rust]
depends_on: [p1-fs-walker, p1-frontmatter-parser, adr-003-waypoints-as-overlays]
created: 2026-05-29
---

`berlin render-waypoint <slug>` renders the projected state at a waypoint:
`projected = base + overlay` (ADR-003).

## Deliverable
- Merge a waypoint's overlay files over the base (overlay shadows same-path
  base file; absent files inherit).
- Refuse to overlay invariant-floor files (root `why.md`, `schema/*`) —
  ADR-003.
- Output the projected tree (to stdout or a target dir).

## Done when
- Rendering the example `waypoint-1-brew-day` produces base + overlay
  correctly.
- An overlay attempting to shadow an invariant-floor file is rejected.

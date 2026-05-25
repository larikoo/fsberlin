---
id: 01JBADR003000000000000000
title: "Waypoints as overlays on a base"
type: adr
adr_number: 3
planning_status: proposed
priority: high
phase: 0
assignee: lari
skills: [architecture, design]
depends_on: []
created: 2026-05-25
---

See body in this folder. Review with the human before promoting to
`planning_status: accepted`.

The decision text below is a draft based on the May 2026 design
conversation. It needs your eye before lock-in.

---


# ADR-003: Waypoints as overlays on a base

Date: 2026-05-25

## Context

Project roadmaps express "what state should this project be in at
future milestone N." The naive approach is to write a roadmap document
that lists target states. But target states drift relative to current
state, and keeping them aligned requires rewriting the roadmap every
time the current state changes.

The better question: *what's the diff between now and milestone N?*
That diff is what a roadmap should encode.

This is the same pattern as Kustomize (Kubernetes), Nix overlays, and
Docker image layers: base + overlay = projected state.

## Decision

Waypoints are folders containing overlay files. Rendering a waypoint
produces the projected state of the project at that milestone:
`projected = base + overlay`.

- The project root holds current state and invariants.
- `waypoints/<slug>/` folders hold overlays — files that override or
  add to the base.
- Files in the waypoint folder shadow files at the root with the same
  relative path.
- Files absent in the waypoint folder inherit from root.
- Cards in a waypoint folder are aliases (symlinks or UUID
  references), with optional overlay fields.

Waypoints are sequential stages of the chosen plan. Git branches are
alternative plans (you can branch the whole project to explore a
different roadmap, each branch with its own waypoint sequence).

## Consequences

**Easier:**
- Roadmaps stay accurate without manual sync.
- The diff between waypoint N and N+1 *is* the planned progress.
- Branching for "what-if" planning is git's job, free.
- Tags on waypoint acceptance create natural milestone snapshots.

**Harder:**
- Rendering requires merge logic; not just file reads.
- Invariant-floor files (why.md, schema) must be barred from overlay
  shadowing; allowing waypoint why.md would let a milestone redefine
  the project.
- Symlinks for card aliases have portability issues (git, Syncthing,
  Windows).

**Committed to:**
- Invariant-floor files cannot be overlaid. Validators enforce this.
- Overlay merge order is well-defined and documented.

## Alternatives considered

- **Full snapshots per waypoint.** Storage waste; drift between
  snapshots and current state.

- **Single roadmap.md.** Simple; doesn't model field-level overrides
  cleanly; rewrites on every state change.

- **Database-backed timeline.** Possible but contradicts ADR-001.

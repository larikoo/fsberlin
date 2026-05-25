---
id: 01JBPHASE0200000000000000
title: "Git wrapper: commit-on-write, role attribution, waypoint tags"
type: phase
phase_number: 2
status: blocked
priority: high
assignee: claude-code
skills: [rust, python, architecture]
depends_on: []
created: 2026-05-25
---

Phase 2 of FSBerlin development.


Add git transactionality to the substrate.

## Deliverables
- Commit-on-write using gitoxide (gix) or libgit2-sys.
- Role attribution: commit author derived from active agent identity.
- Waypoint tags created on `berlin reach-waypoint <slug>`.
- Signed-commit verification (stub — verification logic, no key
  management yet).
- Last-known-SHA passing in MCP writes (used by Phase 4).

## Success criteria
- Every card change produces exactly one commit.
- Commits show role-attributed authorship visible in `git log`.
- Reaching a waypoint creates an annotated git tag.
- Unsigned commits to invariant-floor paths are flagged (blocking
  comes in Phase 3 validators).

## Depends on
- Phase 1.

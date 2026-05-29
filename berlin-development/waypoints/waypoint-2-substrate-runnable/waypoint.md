---
id: 01JBWP2-SU0000000000000
slug: waypoint-2-substrate-runnable
title: "Substrate runnable: phases 0-3 done"
type: waypoint
status: active
criteria: [phase-0-spec-and-schema, phase-1-substrate-core, phase-2-git-integration, phase-3-validators]
created: 2026-05-25
---


# Waypoint 2: Substrate runnable

At this waypoint, phases 0-3 are done:
- Rust substrate core works.
- Git transactionality is in place.
- Validators are deterministic and shared between CLI and (future) MCP.
- `berlin init`, `berlin validate`, `berlin watch`, `berlin query`
  all function on a real project.

No MCP yet, no agents. The substrate alone is useful: a sophisticated
text-based project manager with strong validation and git integration.

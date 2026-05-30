---
id: 01JBADR004000000000000000
title: "Agents are users; models are runtime"
type: adr
adr_number: 4
planning_status: accepted
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


# ADR-004: Agents are users; models are runtime

Date: 2026-05-25

## Context

Multiple AI models exist, and the set changes monthly. If a project's
history records work as done by "Claude Opus 4.7," then six months
later — when Opus 5 is current — the history retroactively becomes
"work done by a model that no longer exists." Threads of reasoning
lose their author. Continuity breaks across model upgrades.

The same problem exists for routing decisions: "run this task on
whichever model is cheap right now" is operationally useful but
smears responsibility across runs.

## Decision

§001 — Agents are first-class users with persistent identity. Each
agent has a role, a `why.md`, a default model, a list of permitted
models, a write scope, and a track record. The agent's identity
persists across model changes; the model is a runtime parameter,
swappable per invocation.

Example:

```yaml
# agents/spymaster.yaml
id: spymaster
role: cross-project consistency checker
why: catches semantic conflicts the validators can't
default_model: gemma-4-local
permitted_models: [gemma-4-local, qwen-3-local, claude-haiku-4-5]
write_scope: [findings/spymaster/]
read_scope: ["**/*"]
```

§002 — On-the-fly model overrides for specific tasks are allowed but
require a stated reason, recorded in the audit log.

§003 — Humans are also agents under this model. They have roles,
scopes, and write attribution. The human-AI distinction becomes
"different agents with different capability profiles," which is the
point of the project lab framing.

## Consequences

**Easier:**
- Project history stays readable across model changes.
- Threads of reasoning have a coherent author.
- Cost/latency tuning is a property of the agent definition, not a
  per-task decision.
- Reproducibility: "rerun with the agent's current setup" is
  meaningful.
- Authority/scope/role decomposition (ADR-005) operates uniformly on
  humans and AIs.

**Harder:**
- Agent definitions become invariant-floor files (ADR-005); changes
  require human signature.
- On-the-fly model overrides need a reason field; small audit burden.

**Committed to:**

C001 — Agent YAML schema is locked at Phase 0.

C002 — Agents have unique stable IDs.

C003 — Models are referenced by name in a registry; the registry is versioned.

C004 — *(Added by ADR-015, 2026-05-30)* Named agents are for **persistent,
recurring roles with track records**. One-shot and periodic operations are
verbs (`berlin distil`, `berlin validate`, etc.), not agents. Do not create
a named agent for an operation that has no continuity to preserve.

## Alternatives considered

- **Model is the user.** Brittle history; loses continuity when
  models change.

- **No agent layer; users pick model per task.** Dissolves
  accountability; comments are attributed to whichever model
  happened to answer; project history becomes unreadable later.

- **Agent layer only for AI; humans handled separately.** Forces two
  authority systems; loses the symmetry that makes the lab framing
  work.

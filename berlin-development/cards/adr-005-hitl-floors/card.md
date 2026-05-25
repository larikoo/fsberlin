---
id: 01JBADR005000000000000000
title: "Two HITL floors enforced by the substrate"
type: adr
adr_number: 5
status: done
priority: high
phase: 0
assignee: lari
skills: [architecture, design]
depends_on: []
created: 2026-05-25
---

See body in this folder. Review with the human before promoting to
`status: accepted`.

The decision text below is a draft based on the May 2026 design
conversation. It needs your eye before lock-in.

---


# ADR-005: Two HITL floors enforced by the substrate

Date: 2026-05-25
Status: Proposed

## Context

Agents are valuable when they can work autonomously. Unbounded
autonomy is dangerous, especially for irreversible actions. Most
AI-tool integrations handle this with per-action confirmation prompts,
which become noise the user clicks through.

The better question: which actions actually require human authority,
and how do we make those gates load-bearing while letting everything
else flow?

## Decision

Two hard floors enforced by the substrate. Between them, agents work
freely.

**Invariant floor:** changes to project invariants — why.md, schema
files, agent definitions, authority assignments — require a
cryptographic human signature. Forgeable git author strings are not
sufficient. The pre-commit hook verifies signatures; unsigned commits
to invariant-floor paths are rejected.

**External-effect floor:** any action that affects reality outside
the project (publishing, sending email, deleting files, merging to
main, calling paid APIs at scale) requires a human approval token.
The MCP server refuses these operations without a referenced token;
the token includes the human signer, the action, the target, and a
timestamp.

Between the floors, agents draft, propose, edit, comment, branch,
run analysis, render views, query the graph, and write memos. No
approval needed.

## Consequences

**Easier:**
- The path of least resistance is the right thing. Agents can't
  accidentally cross floors because the substrate refuses.
- Human attention is preserved for decisions that matter, not
  per-action prompts.
- Audit log is meaningful: floor-crossings are signed events.

**Harder:**
- Signed commits become mandatory; tooling must support GPG/SSH
  signing on every platform.
- Approval tokens require a signing UI (CLI + web).
- Some legitimate workflows feel slower until the user has signing
  configured.

**Committed to:**
- The floors are non-negotiable per-project. A project can configure
  which actions count as external-effect, but cannot remove the
  requirement.
- No AI may sign anything. Signatures are human-only.
- "Goodwill enforcement" is not acceptable; floors are cryptographic.

## Alternatives considered

- **Per-action confirmation prompts.** Become noise; users develop
  banner blindness; high-risk actions get clicked through.

- **No gates; trust agents.** Untenable for irreversible actions and
  for invariants the project depends on.

- **AI as gate.** AI agents enforcing AI safety has the wrong trust
  root (ADR-006). Prompt injection makes AI gates manipulable.

- **Single gate at all writes.** Tedious; smears the signal from
  actions that actually matter.

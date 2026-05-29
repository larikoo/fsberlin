---
id: 01JBADR006000000000000000
title: "Validators mechanical; Spymaster advisory"
type: adr
adr_number: 6
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


# ADR-006: Validators are mechanical; Spymaster and Sentinel are advisory

Date: 2026-05-25

## Context

FSBerlin needs to catch invalid states (broken YAML, missing
references, illegal transitions) and undesirable semantic patterns
(a card's why disagrees with its success criteria; two cards claim
the same scarce resource). The naive instinct is to route everything
through an LLM because LLMs can reason about both.

This instinct is wrong. LLMs are stochastic, expensive,
prompt-injectable, and slow. Using them to enforce invariants that a
parser could check is a category error and sets a precedent ("when in
doubt, ask the LLM") that erodes substrate reliability.

## Decision

§001 — Two layers of checking, with a hard separation.

§002 — **Validators (the mechanical floor):**
- Schema validation
- Reference resolution (UUIDs exist)
- State transition validation
- Path traversal blocking
- YAML safe-load
- Secret scanning (gitleaks)
- Signed-commit verification

Validators are deterministic, blocking, and identical across the
pre-commit hook (for editor users) and the MCP write path (for AI
users). No LLM is in this path. Implemented in Rust.

§003 — **Spymaster and Sentinel (the advisory layer):**
- Spymaster: cross-card semantic conflicts.
- Sentinel: security-focused semantic patterns (PII drift, authority
  drift, anomalous access).

Both are scheduled, read-only, sandboxed processes (Python, separate
container). They produce findings in `findings/spymaster/` and
`findings/sentinel/`. They never edit cards. Findings include their
evidence (which cards were read). Humans promote findings to issues.

## Consequences

**Easier:**
- Trust root is deterministic. AI mistakes can produce false memos
  but cannot corrupt substrate state.
- LLM costs are bounded; mechanical checks are free.
- Validators are unit-testable.
- Spymaster and Sentinel can run on small local models.

**Harder:**
- Tempting to route ambiguous checks to Spymaster; discipline
  required to find mechanical formulations first.
- Two distinct code paths (Rust validators, Python advisors); seam
  requires clean design.

**Committed to:**
- No mechanical check routes through an LLM, ever.
- Spymaster and Sentinel have read-only filesystem access.
- Findings carry their evidence; un-evidenced findings are bugs.

## Alternatives considered

- **LLM-only checking.** Stochastic, expensive, manipulable. Worst of
  all options.

- **Validators only, no semantic layer.** Misses cross-card
  incoherence and security drift; the semantic layer is real value.

- **Combined LLM + validator pipeline.** Couples them; LLM errors
  could mask validator failures.

- **AI with write veto.** Considered for security; rejected because
  LLMs cannot be the trust root for the thing checking LLMs.

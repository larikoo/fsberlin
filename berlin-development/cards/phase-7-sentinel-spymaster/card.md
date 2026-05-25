---
id: 01JBPHASE0700000000000000
title: "Sentinel + Spymaster: scheduled read-only advisors"
type: phase
phase_number: 7
criteria: []
priority: high
assignee: claude-code
skills: [rust, python, architecture]
depends_on: []
created: 2026-05-25
---

Phase 7 of FSBerlin development.


Implement the two scheduled advisory agents.

## Deliverables
- Spymaster (semantic consistency checker, Python).
- Sentinel (security advisor, Python).
- Both run in a dedicated sandboxed container, no network access.
- Both write to `findings/spymaster/` and `findings/sentinel/`.
- Promotion flow: `berlin promote-memo <finding-id>` converts a
  memo to a card and links to the source finding.

## Success criteria
- Spymaster runs on schedule and produces evidence-backed findings.
- Sentinel detects at least these patterns in a test fixture:
  unusual authority change, PII pasted into a card, secret-like
  string in plaintext.
- Neither agent can write to cards/ directly (verified by the scope
  system).

## Depends on
- Phase 6.

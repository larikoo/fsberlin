---
id: 01JBPHASE0500000000000000
title: "Authority enforcement and HITL approval tokens"
type: phase
phase_number: 5
criteria: []
priority: high
assignee: claude-code
skills: [rust, python, architecture]
depends_on: []
created: 2026-05-25
---

Phase 5 of FSBerlin development.


Implement role/scope enforcement and HITL approval tokens.

## Deliverables
- Role-aware MCP gateway: each tool checks the caller's scope.
- Approval token mechanism for external-effect operations.
  - Token = signed JSON with action, target, signer, timestamp.
  - Validated by both the MCP server and any external-effect
    integrations.
- CLI: `berlin sign-approval <action> <target>` produces a token.
- Audit log of token issuance and use.

## Success criteria
- Agents cannot cross HITL floors regardless of how they're prompted.
- Tokens have a clear, signed audit trail.
- Invariant-floor writes require signed commits AND a token (belt and
  braces).

## Depends on
- Phases 3, 4.

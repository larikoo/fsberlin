---
id: 01JBPHASE0300000000000000
title: "Mechanical validators: schema, refs, transitions, paths, secrets"
type: phase
phase_number: 3
status: blocked
priority: high
assignee: claude-code
skills: [rust, python, architecture]
depends_on: []
created: 2026-05-25
---

Phase 3 of FSBerlin development.


Implement the mechanical validator floor.

## Deliverables
- `berlin-validators` crate, shared by CLI/MCP/pre-commit.
- YAML safe-load.
- JSON Schema or YAML schema validation against
  `schema/*.schema.yaml`.
- UUID reference resolution (depends_on, blocks, linked).
- State transition validation.
- Path traversal blocking.
- Frontmatter size limits.
- Gitleaks integration for secret scanning.
- Signed-commit verification (now blocking on invariant-floor paths).
- Installable pre-commit hook (`berlin install-hook`).

## Success criteria
- Invalid states cannot reach disk via either CLI/MCP write or git
  commit.
- Pre-commit hook rejects bad commits with clear messages.
- Validators are unit-tested with >90% coverage.

## Depends on
- Phases 1, 2.

---
id: 01JBPHASE0600000000000000
title: "Python agent runtime: load agent YAML, route to model, audit"
type: phase
phase_number: 6
criteria: []
priority: high
assignee: claude-code
skills: [rust, python, architecture]
depends_on: []
created: 2026-05-25
---

Phase 6 of FSBerlin development.


Implement the Python agent runtime.

## Deliverables
- `fsberlin-agents` Python package.
- Agent loader: reads `agents/<id>.yaml`, validates against schema.
- Model router: dispatches to Ollama or Anthropic API based on
  permitted_models.
- MCP client wired to the Rust MCP server.
- Scope enforcement at runtime (refuses out-of-scope tool calls).
- Audit logging of every model call (which model, which agent, which
  tokens in/out).

## Success criteria
- A YAML-defined agent can be invoked and its outputs land only in
  its write_scope.
- Model swap (Ollama Gemma -> Anthropic Haiku) requires no agent
  code change.
- Audit log is queryable through FSBerlin.

## Depends on
- Phase 4.

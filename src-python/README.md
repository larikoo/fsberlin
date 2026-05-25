# src-python/

Python package for the FSBerlin agent runtime.

Status: skeleton. Implementation begins at Phase 6 (agent runtime).

The agent runtime is a separate process from the Rust substrate. It
connects to the substrate via MCP. This separation lets the substrate
stay tightly focused on file/git/index operations while the agent layer
handles the LLM-adjacent concerns where Python's ecosystem is stronger.

Agents implemented here:
- Spymaster (semantic consistency checker)
- Sentinel (security advisor)
- Custom agents loaded from `agents/*.yaml` in target projects

All agents are read-only against cards; they write only to
`findings/<agent>/`. See ADR-006.

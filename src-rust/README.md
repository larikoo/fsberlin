# src-rust/

Rust workspace for the FSBerlin substrate core.

Status: skeleton. Implementation begins at Phase 1 (substrate core).

See `berlin-development/cards/phase-1-substrate-core/` for the work
ticket. See `docs/ARCHITECTURE.md` for the component diagram.

Crates planned:
- `berlin-core` — library: FS layout, frontmatter parsing, UUIDs, links
- `berlin-cli` — CLI binary (`berlin` command)
- `berlin-mcp` — MCP server binary using rmcp
- `berlin-validators` — shared validator library (used by CLI, MCP, and
  pre-commit hook)

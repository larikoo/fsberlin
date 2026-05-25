---
id: 01JBADR007000000000000000
title: "Editor as peer with MCP"
type: adr
adr_number: 7
planning_status: proposed
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


# ADR-007: Editor as peer with MCP

Date: 2026-05-25

## Context

Power users want to edit cards in VS Code, vim, Obsidian, or any text
editor. AI agents want to edit via MCP tools. If editor users are
second-class, FSBerlin has recreated the very problem it's solving —
humans bolted on as an integration instead of treated as peers.

Editor edits and MCP edits must produce equivalent results. The same
validators must apply. The same constraints must hold. Neither path
can produce a state the other path would reject.

## Decision

Editor edits and MCP edits share a single write path. Validators run
on both:

- For editor users, validators are pre-commit hooks. `lab validate`
  is also exposed as a CLI command for manual runs.
- For MCP users, validators run as a write-path callback before the
  file hits disk.

Both call the same Rust library code. There is no "MCP-only" check.

Frontmatter is human-writeable: short keys, intuitive values, YAML
comments tolerated, key order irrelevant. Every field is something a
human could write in vim. The exception is the UUID, generated once
and ignored visually thereafter.

Conflicts are git's job. Last-known-commit-SHA is included in MCP
writes; mismatches are rejected with a clear message so the agent
re-reads. YAML merges cleanly when fields are one-per-line.

The watcher tolerates broken parses transiently. A file that fails
YAML parse during a watcher tick is logged but does not corrupt the
SQLite index — the previous valid row is retained. Mid-save states
debounce out.

## Consequences

**Easier:**
- Vim users, Obsidian users, AI agents are all first-class.
- Power users get ripgrep and their familiar editor.
- Conflict handling is git, not a custom protocol.
- Adoption is downhill: existing workflows work.

**Harder:**
- Pre-commit hooks must be installable on every platform.
- Atomic-write semantics vary by editor; some don't atomic-rename.
- Watcher debouncing must absorb editor save bursts.

**Committed to:**
- The validator library is the canonical check. CLI, pre-commit hook,
  and MCP callback are thin wrappers.
- Frontmatter is part of the public API. Schema changes are breaking.

## Alternatives considered

- **MCP-only writes.** Locks out vim users; forces tool dependency.
  Defeats the substrate-not-application premise.

- **Custom merge protocol.** Reinvents git poorly.

- **Special "edit mode" required.** Adds ceremony; the substrate
  should accept edits whenever and however they arrive.

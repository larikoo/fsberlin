---
id: 01JBP1WATCHER0000000000000
title: "File watcher: debounce, atomic-rename, transient tolerance (+ berlin watch)"
type: card
building_status: done
priority: high
phase: 1
assignee: claude-code
skills: [rust]
depends_on: [p1-sqlite-mirror, adr-007-editor-as-peer]
created: 2026-05-29
---

Keep the SQLite mirror live as files change, tolerating editor save bursts
(ADR-007). Includes the `berlin watch` CLI verb.

## Deliverable
- `notify`-based watcher; 200ms debounce; atomic-rename aware.
- Transient broken parses are logged but do not corrupt the index — the
  previous valid row is retained (ADR-007 §004).
- `berlin watch [path]` runs it against a project.

## Done when
- Editing a card in vim updates the index within ~250ms.
- A vim/VS Code save burst produces no partial-parse index corruption.
- A file saved mid-write (transiently invalid) leaves the prior row intact,
  then updates once valid.

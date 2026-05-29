---
id: 01JBADR001000000000000000
title: "Filesystem as substrate"
type: adr
adr_number: 1
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


# ADR-001: Filesystem as substrate

Date: 2026-05-25

## Context

Project management systems typically store data in opaque backends:
a Postgres schema, a proprietary block store, or a cloud-only API.
Humans interact through web UIs; AIs interact through APIs that someone
has to design, document, authenticate, and rate-limit. The data isn't
grep-able, isn't visible in the editor, isn't version-controllable
without exporters, and isn't usable when the service is down.

For human-AI project labs, this asymmetry has a specific cost. AIs are
fluent in filesystem operations — `ls`, `cat`, `rg`, walking trees,
parsing YAML — but every PMS forces them to learn a custom API instead.
Humans are fluent in their editor and git, but the canonical project
state lives elsewhere, so they keep a shadow copy in markdown.

## Decision

§001 — FSBerlin uses the filesystem as the primary data substrate. A
project is a folder; the folder's contents are the canonical state.

§002 — Plain text and YAML files are the data format.

§003 — Git is the audit log, branching, and synchronization mechanism.

§004 — No proprietary store, no required server, no required UI. The
filesystem on disk is what FSBerlin manages and what every tool reads.

## Consequences

**Easier:**
- AIs read the substrate natively, no API to design.
- Humans edit in any tool — VS Code, vim, Obsidian, mobile editors.
- Git provides version control, branching, attribution, and audit log
  for free.
- Backup is `cp -r`. Sync is git or Syncthing.
- No vendor lock-in. The project survives FSBerlin's deletion.
- Search via ripgrep is fast and universal.

**Harder:**
- No ACID transactions; the substrate must layer atomicity on top.
- Querying requires an index (SQLite cache); the index can drift if
  the watcher fails.
- Mobile UX is awkward — native filesystem access is poor on iOS.
- Concurrent writes need explicit conflict handling.
- Some operations (full-text search across thousands of cards) are
  slower than a database would be.

**Committed to:**
- The filesystem layout is part of the public API. Changes are
  breaking changes.
- Tooling cannot assume a specific OS filesystem (case-sensitivity,
  path separators, attribute support).
- The SQLite cache is a cache, not a store. It can always be
  regenerated.

## Alternatives considered

- **SQLite as primary store.** Faster queries; harder to edit in any
  tool; requires a custom export to make data portable. Rejected
  because editor-as-peer (ADR-007) becomes impossible.

- **Cloud database (Postgres, DynamoDB).** Solves concurrency cleanly;
  contradicts local-first goals and creates vendor dependency.
  Rejected.

- **Block-based stores (Notion-style).** Rich data model; opaque to
  external tools; requires custom UI. Rejected because the goal is
  to be substrate, not application.

- **Markdown files with no schema layer.** Simplest possible; doesn't
  support relations, queries, or structured fields. Rejected because
  the "P" in PMS requires structure.

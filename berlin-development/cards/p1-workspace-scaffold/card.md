---
id: 01JBP1SCAFFOLD000000000000
title: "Cargo workspace + berlin-core / berlin-cli crates"
type: card
building_status: done
priority: high
phase: 1
assignee: claude-code
skills: [rust]
depends_on: [adr-001-filesystem-as-substrate]
created: 2026-05-29
---

Foundation for the Phase 1 substrate core. A Cargo workspace with the two
crates everything else builds on.

## Deliverable
- Cargo workspace at repo root (or `src-rust/`) with `berlin-core` (library)
  and `berlin-cli` (binary) crates.
- Shared error type (`thiserror`), `Result` alias; no `unwrap()` in library
  code (CLAUDE.md style).
- License headers on every source file (AGPL-3.0-or-later).
- `cargo fmt` clean, `cargo clippy -- -D warnings` clean.

## Done when
- `cargo build` and `cargo test` succeed on an empty workspace.
- `berlin-cli` produces a `berlin` binary that prints `--help`.
- CI-equivalent checks (fmt, clippy) pass locally.

# CLAUDE.md — Instructions for Claude Code (and any other AI agent working in this repo)

You are implementing FSBerlin, a file-hierarchy-based project management
substrate for human-AI collaboration. **FSBerlin is dogfooded:** its own
development is managed using FSBerlin itself, in `berlin-development/`.

## Required reading, in order

1. `README.md` — what FSBerlin is
2. `docs/why.md` — why it exists
3. `docs/SPEC.md` — the specification
4. `docs/adr/` — every ADR with `status: accepted`
5. `berlin-development/cards/<the card you are working on>/`

Do not skip step 4. The ADRs are load-bearing. Architectural choices in this
project are non-obvious and the ADRs document why the defaults were rejected.

## Hard rules

1. **Do not invent fields not in the schema.** If a field is missing,
   extend `schema/*.schema.yaml` first via a new ADR.
2. **Do not route mechanical checks through an LLM.** See ADR-006.
   Validators are deterministic. Spymaster and Sentinel are advisory only,
   never blocking.
3. **Do not implement features outside the current phase.** Phases are
   sequenced in `berlin-development/cards/phase-N-*/`. Phase N is blocked
   until phase N-1 is `done`.
4. **One card per PR.** Reference the card's slug in the commit message.
5. **Update card status as work progresses.** `pending → in-progress → review → done`.
   Status changes are commits.
6. **Write tests before implementation.** Especially for validators —
   deterministic code needs deterministic tests.
7. **Run validators before pushing.** They are also pre-commit hooks once
   Phase 3 lands.
8. **Spymaster and Sentinel never have write access to cards.** They write
   to `*-findings/` directories only. See ADR-006.
9. **Do not read or scan `.beads/` or `.git/`.** These belong to adjacent
   tools. See ADR-008.

## When in doubt

- **If the spec is ambiguous:** propose an ADR. Do not silently interpret.
- **If the schema doesn't cover something:** extend the schema via ADR. Do
  not add ad-hoc fields.
- **If you're tempted to add a feature "while you're in there":** stop. The
  card defines the scope. Out-of-scope work is a separate card.
- **If a check feels like it needs an LLM:** it doesn't. Find the
  mechanical formulation, or route the finding to Sentinel as a memo.

## Style

**Rust:** standard Rust 2024 idioms. `cargo fmt` before commit. `cargo clippy
-- -D warnings` clean. Prefer `Result` over panics. No `unwrap()` in
library code; `expect()` with a message in tests is fine.

**Python:** ruff + black. Type hints required. Python 3.12+.

**YAML:** safe-load only. No `!python/object`. Schema-validated on load.

**Commits:** imperative present tense. First line < 72 chars. Reference card
slug. Sign with GPG/SSH when the substrate supports it (post Phase 2).

## Self-management

`berlin-development/` is an FSBerlin project. Every change to FSBerlin is
recorded as a card there. When you complete a card:

1. Update its `status:` to `done`.
2. Add a `comments/` entry with the implementing commit SHA and a one-line
   summary.
3. Check whether any waypoints should advance.
4. If you made a non-obvious decision, draft an ADR.

The recursion is the test. If FSBerlin can't manage FSBerlin's own
development, the substrate has a hole. Find it and fix it as a card.

## What is NOT in this repo

- Secrets, tokens, credentials. Use `.env` (gitignored) for local dev.
- User data. Examples and dogfooding only.
- Compiled binaries. `target/`, `dist/`, `__pycache__/` are gitignored.

## License headers

Every source file (Rust, Python, shell) begins with:

```
// Copyright (c) 2026 Lari Kemiläinen and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later
```

Documentation files do not need headers; the repo-level `LICENSE-DOCS`
covers them.

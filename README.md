# FSBerlin

*Working title. Field Station Berlin, or File System Berlin — interpretation is open.*

A project management substrate for human-AI collaboration. Cards are folders.
Waypoints are overlays. Agents are users. The filesystem is the lingua franca.

**Status:** pre-alpha. Specification phase. No working code yet.

## What it is

FSBerlin is a small, self-hosted, file-hierarchy-based project management
substrate designed for project labs where humans and AI agents collaborate as
peers. Unlike PMS tools that bolt AI on through narrow APIs, FSBerlin treats
the filesystem as the shared substrate that both humans and AIs read and
write natively. AIs use `ls`, `cat`, and `rg`; humans use any editor; both
use the same MCP server when richer operations are needed.

The substrate ships as a single container with one volume mount. The volume
*is* the project. Backup is `cp -r`. Sync is git or Syncthing. No vendor
lock-in.

## What's here

- `docs/SPEC.md` — the specification (in progress)
- `docs/ARCHITECTURE.md` — the architecture document
- `docs/why.md` — why this project exists
- `docs/adr/` — ADR format guide and conventions (the ADRs themselves live
  as cards in `berlin-development/cards/adr-*/`)
- `schema/` — YAML schemas for cards, agents, projects, waypoints
- `src-rust/` — Rust substrate core (planned)
- `src-python/` — Python agent runtime (planned)
- `examples/sample-project/` — a minimal FSBerlin project showing the layout
- `berlin-development/` — FSBerlin managed as itself (dogfooding)

## How to read this repo

1. Start with `docs/why.md` — the motivation in plain language.
2. Skim `docs/SPEC.md` for the architecture overview.
3. Read the ADRs in `berlin-development/cards/adr-*/` in order — each captures
   one architectural commitment (`docs/adr/README.md` explains the format).
4. Look at `examples/sample-project/` to see what a real FSBerlin project looks like on disk.
5. Look at `berlin-development/` to see FSBerlin used to manage itself.

## License

Code (Rust, Python, shell, schemas): **AGPL-3.0-or-later**.
Documentation, ADRs, prose: **CC BY-SA 4.0**.

See `LICENSE-CODE` and `LICENSE-DOCS`.

## Contributing

The project is in specification phase. The most valuable contributions right
now are review of the ADRs and the SPEC — does the architecture survive
contact with other minds? Open an issue or a PR against an ADR with the
"Alternatives considered" section extended.

Implementation phases follow Phase 0 (spec lock) and proceed in dependency
order. See `berlin-development/cards/` for the work breakdown.

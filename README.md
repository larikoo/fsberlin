# FSBerlin

*Working title. Field Station Berlin, or File System Berlin — interpretation is open.*

A project management substrate for human-AI collaboration. Cards are folders.
Waypoints are overlays. Agents are users. The filesystem is the lingua franca.

**Status:** pre-alpha. Spec locked (Phase 0). Substrate core built and tested
(Phase 1): the `berlin` CLI does `init`, `validate`, `query`, `watch`, and
`render-waypoint`, backed by a filesystem walker, a frontmatter parser, a
slug-based link resolver, and a regenerable SQLite index.

> ⚠️ **Do not dev and drive.** FSBerlin is usable from a phone, in single
> keystrokes — which is rather the point, but it's mobile-friendly, not
> road-friendly. Review from a safe stop.

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

- `docs/SPEC.md` — the specification (core locked at Phase 0)
- `docs/ARCHITECTURE.md` — the architecture document
- `docs/why.md` — why this project exists
- `docs/adr/` — ADR format guide and conventions (the ADRs themselves live
  as cards in `berlin-development/cards/adr-*/`)
- `schema/` — YAML schemas for cards, agents, projects, waypoints
- `src-rust/` — Rust substrate core: `berlin-core` library + `berlin-cli` (Phase 1, working)
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
6. Try it: `cd src-rust && cargo run -p berlin-cli -- validate ../berlin-development`
   (also `query`, `init`, `watch`, `render-waypoint`).

## License

Code (Rust, Python, shell, schemas): **AGPL-3.0-or-later**.
Documentation, ADRs, prose: **CC BY-SA 4.0**.

See `LICENSE-CODE` and `LICENSE-DOCS`.

## Contributing

Phase 0 (spec lock) and Phase 1 (substrate core) are done; implementation
continues in dependency order. The most valuable contributions are still
review of the ADRs and the SPEC — does the architecture survive contact with
other minds? — and now also kicking the tyres on the `berlin` CLI against a
project of your own. Open an issue, or a PR against an ADR with the
"Alternatives considered" section extended.

See `berlin-development/cards/` for the phase-by-phase work breakdown.

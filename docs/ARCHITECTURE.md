# FSBerlin Architecture

*Companion to SPEC.md. Where SPEC.md describes what, this document
describes how the pieces fit.*

## Components

```
┌─────────────────────────────────────────────────────────────────┐
│                          User / Agent                            │
│  (vim, VS Code, Claude Code via MCP, Spymaster, Sentinel, ...)   │
└────────────┬───────────────────────┬────────────────────────────┘
             │                       │
   direct file edit              MCP call
             │                       │
             ▼                       ▼
┌────────────────────────────────────────────────────────────────┐
│                      Write Path (shared)                        │
│                                                                 │
│     ┌─────────────┐    ┌─────────────┐   ┌───────────────┐     │
│     │  Validators │ ─▶ │ Path canon. │ ─▶│ Git commit-on │     │
│     │  (Rust)     │    │ Frontmatter │   │ -write (Rust) │     │
│     └─────────────┘    └─────────────┘   └───────────────┘     │
└────────────────────────────┬───────────────────────────────────┘
                             │
                             ▼
┌────────────────────────────────────────────────────────────────┐
│              Filesystem (the substrate, source of truth)        │
│                                                                 │
│  cards/<slug>/card.md   waypoints/<slug>/overlay.md             │
│  agents/<name>.yaml     findings/                               │
└────────────────────────────┬───────────────────────────────────┘
                             │
                   inotify / fsevents
                             │
                             ▼
┌────────────────────────────────────────────────────────────────┐
│               FS Watcher (Rust) — debounced 200ms                │
└────────────────────────────┬───────────────────────────────────┘
                             │
                             ▼
┌────────────────────────────────────────────────────────────────┐
│            SQLite mirror (regenerable cache, Rust rusqlite)      │
└────────────────────────────┬───────────────────────────────────┘
                             │
                        query API
                             │
                             ▼
┌────────────────────────────────────────────────────────────────┐
│                  MCP Server (rmcp, Rust)                         │
│                                                                 │
│   tools: list, get, create, update, link, query, render,        │
│          snapshot, promote-memo, sign-approval                   │
└────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────┐
│            Agent runtime (Python) — separate process(es)        │
│                                                                 │
│   Spymaster (semantic checker)   Sentinel (security advisor)    │
│   Custom agents per project YAML                                │
│                                                                 │
│   Reads via MCP. Writes only to findings/. No card writes.      │
└────────────────────────────────────────────────────────────────┘
```

## Key design properties

1. **Filesystem is source of truth.** SQLite mirror is regenerable.
   Deleting the cache file and restarting the watcher reconstructs it.
   This means corruption of the cache is harmless; corruption of the
   filesystem is git-recoverable.

2. **Validators run on both paths.** A direct vim save triggers them via
   pre-commit hook. An MCP write triggers them via callback before the
   file hits disk. Same code, two entry points.

3. **Rust core, Python agents.** The substrate (file watching,
   validation, SQLite, git, MCP server) is Rust. The agents (Spymaster,
   Sentinel, custom roles) are Python. The seam between them is the MCP
   protocol — language-agnostic by design.

4. **Agents have asymmetric access.** They can read everything; they can
   only write to scoped paths (typically `findings/<agent>/`). Card
   writes go through human-approval flows.

5. **HITL floors are substrate-enforced.** The Rust write path checks
   signatures before accepting changes to invariant-floor files. The MCP
   tools refuse external-effect operations without approval tokens. No
   agent enforcement is required; the substrate is the gate.

## Container layout

The shipping form is a single Docker container with one volume mount:

```
docker run -v ./my-project:/data ghcr.io/lari/fsberlin:latest
```

Inside the container:

- Rust binary at `/usr/local/bin/berlin` — runs the watcher, validator,
  git wrapper, and MCP server (stdio or SSE).
- Python venv at `/opt/fsberlin/agents/` — separate processes, started by
  the Rust supervisor if `agents/<name>.yaml` is present.
- SQLite cache at `/data/.fsberlin/index.sqlite` — regenerated on start.

The volume `/data` is the only state. Removing the container loses
nothing. Backing up `/data` backs up the project.

See ADR-001 through ADR-008 for the decision rationale behind each of
these properties.

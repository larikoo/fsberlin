# FSBerlin Specification

**Status:** draft, in progress. Locks at end of Phase 0.

This document is the canonical specification. ADRs in `docs/adr/` are the
decision records that justify each section. If this document and an ADR
disagree, the ADR is authoritative until this document is updated.

## 1. Overview

FSBerlin is a project management substrate where:

- The data store is the filesystem (ADR-001).
- A card is a folder with frontmatter, plus identity, relations, index,
  and transactionality layered on top (ADR-002).
- Waypoints are overlay folders that diff against a base (ADR-003).
- Agents are first-class users with persistent identity; models are
  swappable runtime parameters (ADR-004).
- Two HITL floors are enforced by the substrate (ADR-005).
- Validators are mechanical and blocking; Spymaster and Sentinel are
  advisory and read-only (ADR-006).
- Editor edits and MCP edits share the same write path (ADR-007).
- Adjacent tools (Beads, .git) are stacked alongside, not absorbed
  (ADR-008).

## 2. Data model

*(To be expanded in Phase 0. See `schema/card.schema.yaml`,
`schema/agent.schema.yaml`, `schema/project.schema.yaml`,
`schema/waypoint.schema.yaml`.)*

### 2.1 Project structure

```
my-project/
├── why.md
├── requirements.md
├── skills.md
├── .fsberlin/
│   ├── config.yaml
│   └── index.sqlite      # regenerable cache
├── cards/
│   └── <card-slug>/
│       ├── card.md       # frontmatter + body
│       ├── why.md
│       ├── comments/
│       └── attachments/
├── waypoints/
│   └── <waypoint-slug>/
│       └── overlay.md
├── agents/
│   └── <agent-name>.yaml
└── findings/
    ├── spymaster/
    └── sentinel/
```

### 2.2 Card frontmatter

*(See `schema/card.schema.yaml` for the authoritative schema.)*

## 3. Operations

*(To be expanded in Phase 0. Lists the MCP tool surface and CLI verbs.)*

### 3.1 CLI verbs

- `berlin init <path>` — create new project
- `berlin validate [path]` — run validators
- `berlin query <expr>` — query the index
- `berlin watch [path]` — start file watcher
- `berlin render-waypoint <slug>` — render projected state at waypoint
- `berlin commit-card <slug>` — explicit commit (otherwise on save)

### 3.2 MCP tools

*(To be specified in Phase 4. ~10 tools matching CLI verbs plus tools for
promoting memos, signing approvals, listing agents.)*

## 4. Validators (the mechanical floor)

*(See ADR-006. To be expanded in Phase 3.)*

Validators are deterministic, blocking, and identical across the pre-commit
hook and the MCP write path. Implementations live in Rust.

Required validators:

- YAML safe-load
- Schema validation against `schema/*.schema.yaml`
- UUID reference resolution (cross-card links)
- State transition validation (no Done → Todo without rationale)
- Path traversal blocking (no `..` escapes from project root)
- Frontmatter character limits
- Secret scanning (gitleaks integration)
- Signed-commit verification (when invariant floor is touched)

## 5. Agents and authority

*(See ADR-004 and ADR-005. To be expanded in Phase 5.)*

## 6. Security

*(See ADR-005 and ADR-006, plus `docs/adr/` for the full security model.
To be expanded in Phase 5.)*

Three layers:

1. **Auditor (mechanical, blocking):** pre-commit hooks. Same code as
   write-path validators.
2. **Sentinel (semantic, advisory):** scheduled read-only agent. Writes
   to `findings/sentinel/`. No write access to cards.
3. **Human approval (authoritative, per-action):** required for
   external-effect operations. Cryptographic token; audit-logged.

No AI has veto power. Spymaster and Sentinel are auditors without
authority.

## 7. Adjacent tools

*(See ADR-008.)*

FSBerlin coexists with Beads, git, and other tools that store their own
state in the project directory. FSBerlin does not index, scan, or read
these directories. Cross-references happen at the human level.

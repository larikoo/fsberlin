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

The substrate stores four kinds of structured entity, each described by a
locked schema in `schema/`. The schema files are authoritative for the exact
field list; this section describes the model.

- **Card** (`schema/card.schema.yaml`) — a folder under `cards/` (ADR-002).
  Every card has a `type`: `card` (work), `adr`, `phase`, or `waypoint`.
  All types share a small set of universal fields (`id`, `title`, `type`,
  `created`, plus optionals like `assignee`, `skills`, `priority`,
  `depends_on`, `blocks`, `linked`, `phase`, `tags`). Each type then carries
  the status shape that fits its nature (ADR-009):
  - work cards → `building_status` (`pending | in-progress | review | done |
    blocked | archived`);
  - ADR cards → `planning_status` (`proposed | in-discussion | accepted |
    superseded | withdrawn`) plus `adr_number`;
  - phase cards → a `criteria` list of card slugs and a `phase_number`; the
    phase's "met" state is *derived*, never stored;
  - waypoint cards → waypoint-specific fields (`slug`, `status`) per
    `schema/waypoint.schema.yaml`, layered on the universal card fields.
- **Agent** (`schema/agent.schema.yaml`) — a YAML file under `agents/`
  describing a first-class user, human or AI (ADR-004). Carries role, `why`,
  default and permitted models, and read/write scopes.
- **Project config** (`schema/project.schema.yaml`) — `.fsberlin/config.yaml`.
  Holds project identity, `schema_version`, stewards, and the `opaque_paths`
  list of tool-owned directories the substrate must not scan (ADR-008).
- **Waypoint** (`schema/waypoint.schema.yaml`) — a milestone overlay folder
  under `waypoints/` (ADR-003). Its `waypoint.md` is a card of
  `type: waypoint`; the overlay files beside it shadow base files and are not
  themselves schema-governed.

**Identity and relations.** Each card's `id` is a ULID, stable and immutable
across renames (ADR-002); the folder name is for humans. Cross-card relations
(`depends_on`, `blocks`, `linked`) are lists in frontmatter, resolved by the
validator against existing cards.

> **Open item (Phase 0):** ADR-002 specifies relations as lists of *UUIDs*,
> but `card.schema.yaml` and current cards use folder *slugs*
> (e.g. `depends_on: [adr-009-two-dimensional-status]`), while the example
> project still uses ULIDs. This must be reconciled by an ADR before the
> data model is final. See `berlin-development/`.

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

All operations on the substrate run through one of two front ends — the CLI
(for humans in a terminal) and the MCP server (for AI agents and editors).
Both are thin wrappers over the same Rust core, and every write passes the
same validators before touching disk (ADR-007). There is no operation that
one front end can perform and the other cannot express.

### 3.1 CLI verbs

- `berlin init <path>` — create new project
- `berlin validate [path]` — run validators
- `berlin query <expr>` — query the index
- `berlin watch [path]` — start file watcher
- `berlin render-waypoint <slug>` — render projected state at waypoint
- `berlin commit-card <slug>` — explicit commit (otherwise on save)

### 3.2 MCP tools

The MCP server (Phase 4) exposes the same operations as the CLI plus the
collaboration primitives agents need. The detailed tool signatures are
specified in Phase 4; the surface is fixed here:

- `validate` — run validators over a path; returns findings (mirrors
  `berlin validate`).
- `query` — query the index (mirrors `berlin query`).
- `read_card` / `write_card` — read or write a card through the shared
  write path. Writes carry the last-known commit SHA so stale writes are
  rejected and the agent re-reads (ADR-007).
- `render_waypoint` — render projected state at a waypoint (mirrors the CLI).
- `list_agents` — list agent definitions and their scopes.
- `promote_finding` — promote a Spymaster/Sentinel memo from `findings/`
  into a card (human-initiated; agents may draft, not promote on their own).
- `request_approval` / `submit_approval` — obtain and attach a human
  approval token for an external-effect operation (ADR-005). No AI may sign.

Every tool runs under the calling agent's `read_scope`/`write_scope`, and
write tools run the validator callback before the file hits disk. The MCP
server never exposes a path that bypasses validation, and never reads an
`opaque_paths` directory (ADR-008).

## 4. Validators (the mechanical floor)

Validators are deterministic, blocking, and identical across the pre-commit
hook and the MCP write path (ADR-006, ADR-007). No LLM is ever in this path.
Implementations live in Rust, in one library that the CLI, the pre-commit
hook, and the MCP callback all call. A write that fails any validator is
rejected before it reaches disk; the failure message is actionable enough
for the writer (human or agent) to fix and retry. Detailed implementation is
Phase 3; the required set and its semantics are fixed here.

Required validators:

- **YAML safe-load** — parse frontmatter with safe-load only; no
  `!python/object` or other tag execution.
- **Schema validation** — validate each entity against the matching
  `schema/*.schema.yaml`, including the type-specialized status fields
  (ADR-009): work cards require `building_status`, ADR cards require
  `planning_status` and `adr_number`, phase cards require `criteria` and
  carry no stored status.
- **Reference resolution** — every value in `depends_on`, `blocks`,
  `linked`, and a phase's `criteria` must resolve to an existing card. No
  dangling references; `depends_on` may not contain self.
- **State transition validation** — terminal transitions (`building_status:
  done`, `planning_status: accepted | superseded | withdrawn`,
  `archived`) require a commit SHA in the card's audit log. A work card's
  `building_status` may not leave `pending` while any ADR in its
  `depends_on` is not `accepted`. `planning_status: superseded` requires
  `superseded_by` to be set and then locks the field.
- **Derived-state guard** — reject a `phase_met` (or equivalent stored
  status) written onto a phase card; phase health is computed on demand from
  the `criteria` cards, never stored (ADR-009).
- **Path traversal blocking** — no `..` escapes from the project root; no
  writes outside the writer's `write_scope`.
- **Frontmatter limits** — `title` ≤ 200 chars; bounded frontmatter size.
- **Overlay-floor guard** — invariant-floor files (root `why.md`,
  `schema/*`) may not be shadowed by a waypoint overlay (ADR-003).
- **Secret scanning** — gitleaks integration; a detected secret blocks the
  write/commit.
- **Signed-commit verification** — when an invariant-floor path is touched,
  require a valid human cryptographic signature (ADR-005).

Validators are unit-tested per ADR-006 and CLAUDE.md (tests before
implementation). Spymaster and Sentinel are *not* validators: they are
advisory, read-only, and never block (section 6).

## 5. Agents and authority

Agents are first-class users with persistent identity (ADR-004). Humans and
AIs are both agents, differing only in capability profile. Each agent is a
YAML file under `agents/` conforming to `schema/agent.schema.yaml`: a stable
`id`, a `role`, a `why`, a `default_model`, a `permitted_models` list, and
`read_scope`/`write_scope` glob patterns. The model is a runtime parameter,
swappable per invocation; on-the-fly overrides are allowed but require a
stated reason recorded in the audit log. Project history therefore stays
readable across model changes — work is attributed to the agent, not to a
model version that may no longer exist.

**Authority** is the pair (scope, floors). An agent may freely act anywhere
inside its `write_scope` and below both HITL floors (ADR-005):

- **Invariant floor** — changes to project invariants (`why.md`, `schema/*`,
  agent definitions, authority assignments) require a human cryptographic
  signature. Forgeable git author strings do not count. No AI may sign.
- **External-effect floor** — any action affecting reality outside the
  project (publishing, email, deleting files, merging to main, paid APIs at
  scale) requires a human approval token referencing the signer, action,
  target, and timestamp.

Between the floors, agents draft, edit, comment, branch, query, render, and
write memos with no approval. The floors are non-negotiable per project: a
project may configure *which* actions count as external-effect, but cannot
remove the requirement. Detailed token format and signing UX are Phase 5.

> **Open item:** user-card data sensitivity and any per-card
> visibility/scope controls beyond `write_scope` are flagged for the Phase 5
> authority design (see `berlin-development/`, the user-card-security card).

## 6. Security

The security model is the authority model (section 5) viewed from the
threat side: it assumes agents can be wrong or manipulated (prompt
injection) and places the trust root in deterministic code and human
signatures, never in an LLM (ADR-005, ADR-006). It has three layers, in
decreasing order of authority and increasing order of intelligence. Phase 5
expands the human-approval tooling; the structure is fixed here.

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

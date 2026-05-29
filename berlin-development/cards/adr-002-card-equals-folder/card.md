---
id: 01JBADR002000000000000000
title: "Card equals folder, plus four layers"
type: adr
adr_number: 2
planning_status: accepted
priority: high
phase: 0
assignee: lari
skills: [architecture, design]
depends_on: []
linked: [adr-010-relation-identity]
created: 2026-05-25
---

> **Amended by ADR-010 (2026-05-29):** §002 below ("relations are lists of
> UUIDs") is superseded — cross-card relations now reference folder *slugs*,
> not ULIDs. The card's own `id` remains an immutable ULID (§001). §003 and
> §004 stand. See `adr-010-relation-identity` (which sets
> `supersedes: [adr-002§002]`).

See body in this folder. Review with the human before promoting to
`planning_status: accepted`.

The decision text below is a draft based on the May 2026 design
conversation. It needs your eye before lock-in.

---


# ADR-002: Card equals folder, plus four layers

Date: 2026-05-25

## Context

Given that the filesystem is the substrate (ADR-001), the next question
is how to model individual project entities — cards, tickets, tasks.
The naive options are: one file per card, one folder per card, or rows
in a database file.

A card needs: identity (stable across renames), structured fields
(status, assignee, skills), relations (depends on, blocks, links to),
free-form content (description, why), attachments, and comments.
Single files struggle with attachments and comments. Single rows lose
the human-readable aspect. Folders fit naturally.

But "card = folder" alone is insufficient. Folders have identity via
path, which breaks on rename. Folders have no relations beyond tree
structure. Folders don't query well. Folders don't enforce atomicity.

## Decision

A card is a folder. Plus four thin layers that make folders behave
like cards:

§001 — **UUID for identity.** Each card folder contains a card.md with
a UUID in its frontmatter. The folder name is for humans; the UUID is
the stable identifier. Renames, moves, and reorganizations don't
break references.

§002 — **Frontmatter for relations.** *(Superseded by ADR-010: relations
reference folder slugs, not UUIDs. The clause as originally written:)*
Cross-card relations (`depends_on:`, `blocks:`, `linked:`) live in YAML
frontmatter as lists of UUIDs. Relations are not encoded in folder
structure.

§003 — **SQLite index for queries.** A `.fsberlin/index.sqlite` file
mirrors frontmatter from all cards, enabling fast queries
("cards where status=in-progress AND skill includes 'rust'"). The
index is regenerable from the filesystem; it's a cache, not a store.

§004 — **Git transactions for atomicity.** Multi-file card updates
commit atomically. A card's "current state" is the state at the last
commit. Mid-write states are invisible to readers.

## Consequences

**Easier:**
- Cards survive renaming, moving between projects, reorganizing.
- Cross-card relations are explicit and queryable.
- Power users can ripgrep the project for any field value.
- Attachments and comments fit naturally as files in the card folder.

**Harder:**
- The four layers require runtime support; can't be done in vim alone.
- Frontmatter must be valid YAML, machine-parseable, and human-editable.
  Tension between these.
- Reference integrity (no orphan UUIDs) requires validator support.

**Committed to:**
- UUIDs are immutable once assigned.
- Folder names are suggestions, not identifiers.
- The SQLite cache must always be derivable from the filesystem.

## Alternatives considered

- **Single file per card with a defined extension** (e.g., `.card`).
  Simpler model, but breaks down for attachments and comments.

- **Path-as-identity.** What you get with naive folder use; ADR
  discusses why this fails on rename.

- **Database rows with file attachments.** Solves the four layers
  natively, but loses editor-as-peer (ADR-007) and grep-ability.

- **Card as a single YAML file with rich content embedded.** Possible
  but makes attachments awkward (base64 embeds bloat the file).

---
id: 01JBADR010000000000000000
title: "Relations reference slugs; ids stay ULIDs"
type: adr
adr_number: 10
planning_status: accepted
priority: high
phase: 0
assignee: lari
skills: [architecture, design]
depends_on: []
linked: [adr-002-card-equals-folder]
created: 2026-05-29
---

Drafted, discussed, and accepted 2026-05-29 (berlin-17v) via a two-phase
triage review of all nine decisions. Amends the relation clause of ADR-002;
it does not supersede ADR-002 as a whole.

---


# ADR-010: Relations reference slugs; ids stay ULIDs

Date: 2026-05-29

## Context

ADR-002 made two commitments that are now in tension with each other and
with practice:

1. Each card has an immutable ULID `id`; the folder name is a human label.
   Renames don't break identity.
2. Cross-card relations (`depends_on`, `blocks`, `linked`) are "lists of
   UUIDs," so that references survive renames too.

The repository contradicts clause 2 three ways:

- `schema/card.schema.yaml` documents relations as "card slugs" and its
  validation rules resolve `depends_on` slugs to cards.
- Every dogfooding card in `berlin-development/` uses slugs
  (`depends_on: [adr-009-two-dimensional-status]`).
- The example project (`examples/sample-project/`) uses ULIDs
  (`depends_on: [01JBSAMPLECARD0000000000001]`).

So the schema disagrees with an accepted ADR, and the two sub-projects
disagree with each other. This blocks locking the schemas (see
`lock-four-schemas`): the schema cannot be called "consistent with the
accepted ADRs" while ADR-002 says one thing and the schema says another.

The deeper question is which form is *canonical* for a relation. ULIDs are
rename-stable but opaque. Slugs are readable but break if a folder is
renamed without updating referrers. The project's own values weigh heavily
here:

- `docs/why.md` promises the substrate is grep-able and editable in any
  editor — "the AI reads the project the way it reads a codebase."
- ADR-007 commits to human-writeable frontmatter: "Every field is something
  a human could write in vim. The exception is the UUID, generated once and
  ignored visually thereafter."

A relation list of opaque ULIDs (`depends_on: [01JB...]`) is precisely the
field ADR-007 says should not exist. `rg "depends_on.*adr-009"` works; the
ULID equivalent does not. The readability cost of clause 2 is paid on every
read, by both humans and agents; the rename cost is paid rarely and can be
mechanized.

## Decision

Slugs are the canonical form for cross-card relations. ULIDs remain the
canonical form for a card's own identity.

Concretely:

- A card's `id` is a ULID, immutable, the substrate's internal identity for
  that card. Unchanged from ADR-002 clause 1.
- `depends_on`, `blocks`, `linked`, and a phase's `criteria` hold **folder
  slugs**, not ULIDs. This amends ADR-002 clause 2.
- Rename stability becomes a substrate responsibility, not a property of the
  reference format. Renaming a card is a substrate operation
  (`berlin mv <old-slug> <new-slug>`) that atomically rewrites every
  referring slug across the project in one commit. Renaming a folder by hand
  in an editor is allowed but leaves dangling slugs until `berlin mv` or the
  validator's repair pass reconciles them.
- The reference-resolution validator (Phase 3) treats an unresolvable slug
  as a blocking error, with the offending card and field named.
- Slugs are unique within a project. Slug uniqueness is a validated
  invariant; the ULID remains the tiebreaker of last resort and the key the
  SQLite index joins on internally.

**Migration:** the example project's ULID relations are rewritten to slugs.
After migration no relation field in any card contains a ULID.

## Consequences

**Easier:**
- Relations are grep-able and human-writeable, honoring `why.md` and ADR-007.
- `depends_on: [adr-009-two-dimensional-status]` is self-documenting; a
  reviewer reading frontmatter sees what it points at without a lookup.
- The schema, the dogfooding cards, and the example project finally agree;
  `lock-four-schemas` can close.
- Identity is decoupled from label. Because the ULID `id` is stable and only
  the slug is human-facing, a card can be renamed or repurposed on the fly —
  new slug, new role — without breaking references or losing its history
  thread. `berlin mv` rewrites the referrers; the ULID carries the
  continuity. The label is mutable; the identity is not.

**Harder:**
- Renames are no longer free. The substrate must own a rename operation that
  rewrites referrers atomically, and the validator must detect dangling
  slugs. Hand-renames in an editor create a transient broken state until
  reconciled.
- Slug uniqueness becomes a validated invariant the substrate must enforce.
- The SQLite index must map slug -> ULID and keep the mapping current as the
  watcher sees renames.

**Committed to:**
- No relation field stores a ULID after migration.
- Slugs are unique per project; `berlin mv` is the supported rename path.
- A card's own `id` ULID is still immutable and is what the index joins on.

## Alternatives considered

- **ULIDs canonical in relations (uphold ADR-002 as written).** Maximally
  rename-stable, but every relation list becomes opaque, contradicting
  `why.md` (grep-ability) and ADR-007 (human-writeable frontmatter). The
  dogfooding cards already voted against this with their feet. Rejected.

- **Dual form: write slugs, store a resolved ULID alongside.** The validator
  resolves slug -> ULID on write and persists both. Survives renames without
  a rewrite pass, but reintroduces an opaque ULID into the file (defeating
  the readability goal) or hides state the file doesn't show (defeating
  grep-ability). More machinery for a rare event. Rejected as premature.

- **Slugs with no rename support.** Simplest, but a folder rename silently
  orphans every referrer with no recovery path. Rejected: the substrate must
  offer a safe rename, not forbid renames or leave them broken.

- **Path-based references.** Encodes tree position into the reference;
  breaks on any reorganization and couples relations to folder layout, which
  ADR-002 explicitly rejected. Rejected.

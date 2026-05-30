---
id: 01JBADR009000000000000000
title: "Status fields are type-specialized, not universal"
type: adr
adr_number: 9
planning_status: accepted
priority: high
phase: 0
assignee: lari
skills: [architecture, design]
depends_on: []
created: 2026-05-25
---

# ADR-009: Status fields are type-specialized, not universal

Date: 2026-05-25

## Context

The current schema has a single `status:` field shared by all card
types: `pending | in-progress | review | done | blocked | archived`.

This is a lowest-common-denominator that fits none of the three card
types well:

- **ADR cards** need a ratification lifecycle: was the decision drafted,
  debated, accepted, superseded? `status: done` on an ADR means the
  draft is written, but not that the architecture is ratified. Two
  practitioners disagree on what it means.

- **Work cards** need an execution lifecycle: is implementation pending,
  underway, in review, done? They don't have a ratification lifecycle —
  if a work card exists, the decision to do the work has already been
  made. Adding a planning field to work cards creates redundant state
  that can drift.

- **Phase cards** are not work items at all. A phase is a gate: it is
  either reached or it isn't. Its "status" is derived from whether its
  constituent cards are complete — there is no independent execution
  lifecycle to track on the phase card itself.

The naive fix — add a second universal field (`planning_status`) — has
the same problem as the first. It forces every card to carry fields
that are irrelevant to its type, creates derived state that can go out
of sync, and still requires someone to update N cards when one ADR is
accepted.

Phase 0's success criteria required "all eight ADRs have `status:
accepted`," but `accepted` is not a valid value in the current schema.
That gap is the symptom.

## Decision

§001 — The universal `status:` field is retired. Each card type gets
the status shape that matches its nature.

§002 — **ADR cards** → `planning_status`:

```yaml
planning_status: proposed | in-discussion | accepted | superseded | withdrawn
```

§003 — **Work cards** (`type: card`) → `building_status`:

```yaml
building_status: pending | in-progress | review | done | blocked | archived
```

§004 — **Phase cards** → `criteria` (a list of card slug references):

```yaml
criteria:
  - adr-001-filesystem-as-substrate
  - adr-002-card-equals-folder
  # ...
```

Phase cards carry no stored status field. Whether a phase is met is
derived by the validator: a phase is met when every card in its
`criteria` list is in its terminal state (`planning_status: accepted`
for ADR cards; `building_status: done` for work cards). This computed
result is never written back to the phase card — it is always
re-derived from the current state of the referenced cards.

§005 — **The handoff from planning to building is structural, not
duplicated state.** A work card references its governing ADR via
`depends_on:`.
The validator enforces: a work card's `building_status` cannot leave
`pending` while any ADR in `depends_on:` has `planning_status` other
than `accepted`. Accepting the ADR is the signal; no field on the work
card needs to change.

§006 — **Validators enforce:**

1. Work card `building_status` cannot leave `pending` while any ADR in
   `depends_on:` has `planning_status: proposed` or `in-discussion`.
2. `planning_status: superseded` locks the ADR card. `superseded_by:`
   must be set; the field may not change again.
3. `phase_met` is derived only — the validator computes it from
   criteria card states. It is never stored on the phase card.
4. Transitions to terminal states require a commit SHA reference in the
   card's audit log.

§007 — **Migration:** `status:` is removed from all existing cards. ADR cards
gain `planning_status:` (mapped from their current state). Work cards
gain `building_status:` (renamed from `status:`). Phase cards gain
`criteria:` lists (replacing the free-text `success_criteria:` field).

## Consequences

**Easier:**
- Phase criteria "all ADRs accepted" is unambiguous: check
  `planning_status: accepted` on each ADR card.
- No synchronization burden: accepting an ADR unblocks work cards
  automatically — no fields on work cards need updating.
- Phase health is always current: derived from card states, never stale.
- Each card type carries only the fields relevant to it.

**Harder:**
- Schema migration is a breaking change (ADR-001: filesystem layout is
  public API). All existing cards must be migrated in a single commit.
- Free-text `success_criteria:` on phase cards must be converted to
  actual card references — every criterion must be a card.
- The validator has three distinct status checks to implement instead
  of one.

**Committed to:**

C001 — `status:` is retired. No card uses it after migration.

C002 — Phase cards never have a stored status field. If you feel the urge to add one, the criterion it would represent should be a card.

C003 — The `depends_on:` field on work cards is the canonical planning provenance. It is not optional when an ADR governs the work.

## Alternatives considered

- **Two fields on every card (`planning_status` + `building_status`).**
  Avoids the type-specialization complexity, but forces every card to
  carry irrelevant fields, duplicates state that can drift, and still
  requires N writes when one ADR is accepted. Rejected: the
  synchronization problem was the original complaint.

- **`adr_status:` field for ADR cards only.** Fixes ADRs but leaves
  phase cards without a coherent model. Rejected.

- **Keep `status:`, add `accepted: true/false`.** Patchwork. Doesn't
  model `in-discussion`, `superseded`, or the derived phase gate.
  Rejected.

- **Use tags for planning status.** Tags are free-form and unvalidated.
  The blocking constraint requires machine-readable structured fields.
  Rejected.

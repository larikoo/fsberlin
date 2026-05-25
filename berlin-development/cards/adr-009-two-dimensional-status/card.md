---
id: 01JBADR009000000000000000
title: "Card workflow has two orthogonal statuses — planning and building"
type: adr
adr_number: 9
planning_status: proposed
building_status: done
priority: high
phase: 0
assignee: lari
skills: [architecture, design]
depends_on: []
created: 2026-05-25
---

# ADR-009: Card workflow has two orthogonal statuses — planning and building

Date: 2026-05-25
planning_status: proposed
building_status: done

## Context

The current schema has a single `status:` field with values:
`pending | in-progress | review | done | blocked | archived`.

This field tries to serve two orthogonal concerns simultaneously:

1. **Planning lifecycle** — has the decision or plan been ratified? Is an
   ADR still under discussion? Is a phase's scope locked? This axis moves
   through `proposed → in-discussion → accepted → superseded/withdrawn`.

2. **Building progress** — is implementation underway? Is the work
   complete? This axis moves through `pending → in-progress → review →
   done | blocked | archived`.

For ADR cards this creates an ambiguity with real consequences: marking
an ADR `status: done` means the drafting task is complete, but says
nothing about whether the architectural decision has been reviewed and
ratified. An ADR can be fully drafted and yet still proposed. Two
practitioners looking at `status: done` on an ADR card will disagree on
what it means.

The same problem applies to phase cards: `status: pending` on a phase
means implementation hasn't started, but doesn't say whether the phase
plan has been reviewed and accepted. A phase can be planned and accepted
(ready to implement) or still open for debate — both look like
`status: pending`.

Phase 0's success criteria required "all eight ADRs have `status:
accepted`," but `accepted` is not a valid value in the current schema.
That gap is the symptom. This ADR is the fix.

## Decision

Cards have two status fields, not one.

```yaml
planning_status: proposed | in-discussion | accepted | superseded | withdrawn
building_status: pending | in-progress | review | done | blocked | archived
```

The existing `status:` field is renamed to `building_status:`. A new
`planning_status:` field is added. The schema is updated in the same
commit that migrates all existing cards.

**Semantics:**

- `planning_status` tracks whether the card's decision or plan has been
  ratified. It answers: "is it safe to build?"
- `building_status` tracks implementation progress. It answers: "how far
  along is the work?"

**Validators enforce:**

1. `building_status` cannot leave `pending` while `planning_status` is
   `proposed` or `in-discussion`. A card in review limbo cannot be
   built against.
2. `planning_status: superseded` locks both fields. A superseded decision
   is frozen; only a new ADR (with a `supersedes:` reference) can change
   that state.
3. Transitions to terminal states (`done`, `accepted`, `superseded`,
   `withdrawn`, `archived`) require a referenced commit SHA in the card's
   audit log. The substrate records which commit carried the transition.

**Card types use the fields differently:**

- **ADR cards:** `planning_status` is the document lifecycle
  (`proposed → in-discussion → accepted`). `building_status` tracks
  drafting work (`pending → done`).
- **Phase cards:** `planning_status` signals whether the phase plan is
  locked (`proposed → accepted`). `building_status` tracks
  implementation.
- **Work cards:** both fields apply normally. A work card with
  `planning_status: accepted` and `building_status: in-progress` is the
  common case.

**This ADR card itself** uses the new schema: `planning_status: proposed`
(under review), `building_status: done` (drafting complete).

## Consequences

**Easier:**
- Phase 0 success criteria "all ADRs have `status: accepted`" becomes
  unambiguous: it means `planning_status: accepted`.
- The validator rule "can't build a proposed plan" is expressible.
- Views can surface planning health and building health independently.
- ADR lifecycle and implementation tracking stop conflating.

**Harder:**
- All existing cards must be migrated: `status:` → `building_status:`,
  plus `planning_status:` added.
- Two fields to maintain instead of one — slightly more ceremony.
- Schema migration is a breaking change to the public API (ADR-001
  committed to the filesystem layout as public API).

**Committed to:**
- `status:` is retired. No new cards use it after migration.
- `planning_status` is present on every card, even work cards where it
  defaults to `accepted` (the plan is to do the work; there is no
  separate ratification step).
- The validator seam (Phase 3) enforces the cross-field constraints.

## Alternatives considered

- **Add an `adr_status:` field for ADR cards only.** Fixes ADRs but
  leaves the same ambiguity on phase cards. Adds a type-specific field
  for a concept that is general. Rejected.

- **Keep `status:`, add `accepted: true/false` boolean.** Patchwork fix.
  Doesn't compose. Doesn't model `in-discussion`, `superseded`, or
  `withdrawn`. The same problem recurs for phases. Rejected.

- **Use tags to express planning status.** Tags are free-form and
  unvalidated. The constraint "can't build while proposed" requires
  machine-readable structured status. Rejected.

- **No planning_status; ADRs use a separate review workflow outside
  FSBerlin.** Breaks the dogfooding premise. FSBerlin must be able to
  represent its own ADR lifecycle. Rejected.

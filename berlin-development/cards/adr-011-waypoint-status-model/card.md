---
id: 01JBADR011000000000000000
title: "Waypoints use derived criteria, like phases"
type: adr
adr_number: 11
planning_status: accepted
priority: medium
phase: 0
assignee: lari
skills: [architecture, design]
depends_on: []
linked: [adr-003-waypoints-as-overlays, adr-009-two-dimensional-status]
created: 2026-05-29
---

Drafted and accepted 2026-05-29 (berlin-e93) via a two-phase triage review
of six decisions (all accepted; #4 accepted with the acyclicity guardrail).
Extends ADR-009's type-specialized model and ADR-003 to waypoints; amends the
locked card schema. Does not supersede either ADR.

---


# ADR-011: Waypoints use derived criteria, like phases

Date: 2026-05-29

## Context

ADR-009 made card status type-specialized but explicitly scoped itself to
three card types (work, ADR, phase). Waypoints were left half-migrated:

- The standalone `schema/waypoint.schema.yaml` still defined free-text
  `success_criteria` (a list of strings) and a `status` enum that mixed a
  human lifecycle (`planned`/`active`/`abandoned`) with a completion value
  (`reached`) — exactly the stored-derived-state ADR-009 warns against.
- Waypoint fields lived in a separate file, while the other three card types
  were documented inline in `card.schema.yaml` — even though `type: waypoint`
  is already a value in that schema's `type` enum.
- The dogfooding waypoints expressed their "done" test as body prose
  ("phases 0-3 done"), which is not machine-checkable and drifts.

ADR-003 already frames a thin waypoint as "a named criteria list," and the
example waypoints naturally think in *phases* ("phases 0-5+8 done"). The
phase model from ADR-009 — `criteria` as slug references, "met" derived on
demand — fits waypoints almost exactly. The open question (berlin-e93) was
whether to finish the migration by applying that model to waypoints.

## Decision

Waypoints adopt the phase model, with one waypoint-specific twist for the
human lifecycle. Six points:

§001 — **`criteria` replaces `success_criteria`.** A waypoint's `criteria` is a
   list of slug references; free-text `success_criteria` is retired for
   waypoints (as it already was for phases).

§002 — **"Reached" is derived, never stored.** A waypoint is *reached* when every
   card in its `criteria` is in its terminal state. The validator computes
   this on demand; it is never written back to the waypoint.

§003 — **`status` carries human intent only: `planned | active | abandoned`.**
   `reached` is no longer a stored status value. The semantics are *path
   membership*, not temporal position:
   - `planned` — speculative / not on the committed roadmap (e.g. a waypoint
     on an exploratory branch);
   - `active` — on the committed roadmap (whether ahead of or behind current
     progress — "reached" tells you which, and it is derived);
   - `abandoned` — removed from the roadmap.
   An optional `reached_date` records when the derived `reached` first became
   true (an audit timestamp, not a status).

§004 — **`criteria` may reference card *or* phase slugs**, so "phases 0-3 done"
   is expressible directly:
   `criteria: [phase-0-spec-and-schema, ..., phase-3-validators]`.
   The terminal test dispatches on the referenced card's type (work →
   `done`, ADR → `accepted`, phase → `phase_met`), which the validator
   already does per ADR-009. **Guardrail (acyclicity):** a waypoint slug may
   NEVER appear in any `criteria` list. Aggregation flows one way —
   cards/ADRs ← phases ← waypoints — keeping the derive-graph a DAG.

§005 — **Fold waypoint fields into `card.schema.yaml`** as a fourth
   `waypoint_fields` section, parallel to `work_fields` / `adr_fields` /
   `phase_fields`. The standalone `schema/waypoint.schema.yaml` is retired
   (deleted). Overlay-file semantics remain governed by ADR-003, referenced
   from the schema as a comment.

§006 — **This is recorded as an ADR** (this one), per CLAUDE.md routing all
   schema changes through an ADR, and applied in the accepting commit
   (migrating the three dogfooding waypoints), mirroring ADR-009's
   accept-with-migration pattern.

§007 — **Migration (in the accepting commit):** `waypoint.schema.yaml` deleted;
`waypoint_fields` added to `card.schema.yaml`; the three berlin-development
waypoints gain `criteria` (phase-slug references) and move
`status: planned/reached` → `status: active` (all are on the committed
roadmap; waypoint-1 keeps its `reached_date` as the derived-reached audit
stamp).

## Consequences

**Easier:**
- Waypoint health is machine-checkable and always current — derived from the
  same card states everything else uses, never a stale prose list.
- "phases 0-3 done" is one line of slug references; it cannot drift from the
  phases it names.
- All four card types are documented in one schema file; `type: waypoint` no
  longer points at a separate, divergent definition.
- The model is uniform: phases and waypoints both derive completion from a
  `criteria` list; only the human-intent `status` differs.

**Harder:**
- Another breaking schema change after the Phase 0 "lock" (justified by this
  ADR; ADR-001 holds that layout changes are breaking and ADR-tracked).
- The validator must dispatch the terminal test by referenced-card type and
  enforce the no-waypoint-in-criteria acyclicity rule (Phase 3).
- `status` and derived `reached` are two axes; tooling/UI must show both
  ("active · reached ✓") rather than a single state.

**Committed to:**
- `success_criteria` is retired everywhere; every criterion is a real card.
- `reached` is derived only; `reached_date` is an audit timestamp.
- No waypoint slug appears in any `criteria` list.

## Alternatives considered

- **Keep free-text `success_criteria`.** Human-readable but not
  machine-checkable, drifts from the work it describes, and contradicts
  ADR-009's "every criterion is a card." Rejected.

- **Keep `reached` as a stored status value.** Simpler (one axis), but stores
  derived state that can go stale — the exact failure ADR-009 set out to
  avoid. Rejected. (Considered a softer form: derived-but-human-confirmed,
  like ADR-009's done-handoff. Folded into the `active` + derived-`reached`
  pairing instead: the human commits the path via `status`, the substrate
  reports arrival via the derivation.)

- **Keep the standalone `waypoint.schema.yaml`.** Less churn, but leaves the
  organization inconsistent (three types inline, one in a separate file) for
  no benefit now that the field model is unified. Rejected.

- **Reference only cards in `criteria`, never phases.** Forces every waypoint
  to enumerate each phase's constituent cards — verbose and drift-prone the
  moment a phase's criteria change. Rejected in favor of allowing phase
  references with the acyclicity guardrail.

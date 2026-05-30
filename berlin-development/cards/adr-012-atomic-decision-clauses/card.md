---
id: 01JBADR012000000000000000
title: "ADRs are made of numbered clauses (§NNN)"
type: adr
adr_number: 12
planning_status: accepted
priority: medium
phase: 0
assignee: lari
skills: [architecture, design]
depends_on: []
linked: [adr-010-relation-identity, adr-011-waypoint-status-model]
superseded_by: [adr-013-committed-to-clauses§005]
created: 2026-05-29
---

Drafted and accepted 2026-05-29 via its own protocol: the nine Decision
clauses below were served one at a time and ratified individually (see
comments/2026-05-29-accepted.md for the review record). This ADR is the first
written in the convention it defines.

---


# ADR-012: ADRs are made of numbered clauses (§NNN)

Date: 2026-05-29

## Context

Two frictions surfaced while reviewing ADR-010 and ADR-011 on a mobile,
text-only device.

First, **amendment was imprecise.** ADR-010 had to amend "ADR-002 clause 2"
in prose, with a hand-written banner and a "the other three layers still
stand" caveat. There was no addressable unit to point at, so supersession was
whole-ADR or hand-waved.

Second, **review didn't decompose.** To triage an ADR we built an HTML page
that *I* hand-split into "6 decisions." That decomposition was lossy,
non-reproducible, and lived in the tool rather than the ADR — and on a phone
the page was both too complex and not atomic enough.

The shared root cause: an ADR's decisions were an undifferentiated block of
prose. The fix is to make the atoms first-class in the source, address them
stably, and serve them one at a time. The unit is a **clause** — a discrete
normative statement — marked `§NNN`.

## Decision

§001 — Every normative statement in an ADR's **Decision** section carries a
marker `§NNN`, unique within that ADR, assigned in order, and never reused or
renumbered (the immutability discipline of ADR numbers and card ULIDs).
Withdrawing a clause marks it withdrawn; it does not free the number.

§002 — A clause is **one independently-acceptable claim**. If you can accept
half of it, it is two clauses. Authors atomize at write time; this is the
forcing-function that keeps review honest.

§003 — Cross-references use the form `ADR-002§002` (ADR number + clause
marker). The validator resolves them like any reference (cf. ADR-010); a
reference to a non-existent ADR or clause is a blocking error.

§004 — Supersession operates at **clause granularity**. A later ADR declares
`supersedes: [adr-002§002]` and replaces that one clause, leaving the rest in
force. The superseded clause is marked superseded (with a pointer to its
replacement), not deleted — append-only.

§005 — *(Superseded by ADR-013§001–§004: "Committed to" consequences now use
`CNNN` clause numbering in their own sequence.)* Only Decision-section
statements are §NNN clauses. Context, Consequences ("Easier"/"Harder"), and
Alternatives stay prose.

§006 — Clause numbering is **flat** (`§007`, never `§004.1`). No nesting; a
would-be sub-clause is two sibling clauses (per §002).

§007 — The eleven existing ADRs (001–011) are **retrofitted** with clause
markers on their Decision sections. Adding `§NNN` labels does not change any
decision, so it is append-only-safe; the payoff is that prior amendments
(notably ADR-010 → `ADR-002§002`) become precise references.

§008 — Reviewing an ADR is a **substrate operation that serves clauses one at
a time**: the backend holds a cursor, takes each verdict, and advances. It is
resumable and queryable. Answers are terse — `y` (accept) or `n` (needs
discussion) — and each verdict is **appended to the review record without
further discussion**; the artifact maps `§ref → y | n + note` and is not a
throwaway. Realized as a CLI verb and a Phase 4 MCP tool; Phase 9 renders the
drip.

§009 — This ADR is written in its own `§`-clause style, making it the first
instance of the convention it defines. If the convention cannot express
itself cleanly, it is wrong.

## Consequences

**Easier:**
- Amendment and supersession become surgical and machine-checkable
  (`supersedes: [adr-002§002]`) instead of prose caveats.
- Review needs no intelligence in the renderer: a clause is the unit; the
  view never shows more than one. The HTML-complexity problem dissolves —
  the answer was atoms + a drip, not a better page.
- Reviews, comments, and Spymaster/Sentinel findings can all target a clause
  precisely (`ADR-007§003`).
- Works on any device, including text-only chat — exactly how this ADR was
  ratified.

**Harder:**
- Authoring discipline: each decision must be atomized and numbered at write
  time.
- A one-time retrofit pass over ADRs 001–011 (§007).
- The validator gains clause-reference resolution and the never-renumber
  invariant to enforce (Phase 3); the `supersedes` field must accept clause
  refs.

**Committed to:**

C001 — Clause numbers (`§NNN`) are immutable and never reused.

C002 — Only Decision-section statements are `§NNN` clauses; "Committed to"
consequences use `CNNN` numbering (ADR-013).

C003 — Clause references resolve or the write is blocked.

## Alternatives considered

- **Keep prose, decompose at review time** (the HTML approach). Lossy,
  non-reproducible, tool-bound, and not atomic in the source. Rejected — it
  was the thing that failed.
- **Whole-ADR supersession only.** Forces "the other clauses still stand"
  hand-waving and reissuing entire ADRs for one-line changes. Rejected.
- **Nested numbering (`§4.1`).** Reintroduces non-atomic units and ambiguous
  references. Rejected in favor of flat (§006).
- **Number everything (Context, Consequences too).** Dilutes the clause set;
  the ratifiable atoms are the Decision statements. The narrower "Committed
  to" question is deferred, not bundled.

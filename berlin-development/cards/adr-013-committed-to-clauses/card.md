---
id: 01JBADR013000000000000000
title: "'Committed to' consequences use C-prefix clause numbering"
type: adr
adr_number: 13
planning_status: accepted
priority: medium
phase: 0
assignee: lari
skills: [architecture, design]
depends_on: []
linked: [adr-012-atomic-decision-clauses]
supersedes: [adr-012§005]
created: 2026-05-30
---

Drafted and accepted 2026-05-30 (berlin-hkr) via a two-clause triage drip.
Amends ADR-012§005, which deferred the question of whether "Committed to"
consequences should be numbered. Does not otherwise change ADR-012.

---


# ADR-013: "Committed to" consequences use C-prefix clause numbering

Date: 2026-05-30

## Context

ADR-012§005 numbers only Decision-section statements as clauses (§001, §002…),
leaving Context, Consequences, and Alternatives as prose — with the question
of "Committed to" consequences explicitly deferred.

"Committed to" is the subset of Consequences that states binding invariants
going forward — not side effects, but pinned constraints. Examples from
existing ADRs:

- ADR-005: "No AI may sign anything."
- ADR-006: "No mechanical check routes through an LLM, ever."
- ADR-010: "No relation field stores a ULID after migration."

These are the statements future implementers and ADR authors actually bump
into. Being able to cite one precisely — rather than "somewhere in ADR-005"
— is the same value that motivated §NNN for Decision clauses.

The open question was namespace: should Committed-to clauses share the §NNN
sequence with Decision clauses, or get their own prefix? Sharing risks
renumbering Decision clauses when commitments are added later (breaking
existing citations). A separate namespace keeps both sequences independently
stable. The `§C` prefix was considered but rejected as two characters; `C`
alone (e.g. `C001`) is sufficient.

## Decision

§001 — "Committed to" consequences receive their own clause numbering using
the prefix `C`: `C001`, `C002`… The sequence is per-ADR and starts at
`C001` regardless of how many Decision clauses the ADR has.

§002 — The Decision clause sequence (`§001`, `§002`…) and the Committed-to
sequence (`C001`, `C002`…) are **independently immutable**: adding a new
commitment never touches the Decision numbering, and vice versa.

§003 — Cross-references use the full form `ADR-NNN§NNN` for Decision clauses
(unchanged from ADR-012) and `ADR-NNNCnnn` for Committed-to clauses —
e.g. `ADR-005C002`. No separator between the ADR number and `C` prefix.

§004 — Only the "Committed to" sub-section of Consequences is numbered.
"Easier" and "Harder" remain prose; "Alternatives considered" remains prose.
This is a narrowing of scope from the deferred question in ADR-012§005, not
a reopening of it.

## Consequences

**Easier:**
- Precise citation of long-lived invariants without pointing at a whole ADR.
- Both clause sequences grow independently without invalidating existing refs.
- The one-character `C` prefix is consistent with the low-friction
  single-keystroke philosophy already embedded in the review protocol
  (ADR-012§008).

**Harder:**
- Authoring ADRs requires numbering two sequences rather than one.
- Retrofit pass needed for existing ADRs 001–012 (low priority; new ADRs
  adopt the convention immediately).

**Committed to:**

C001 — Decision clauses use `§NNN`; Committed-to clauses use `CNNN`. No
other sections are numbered.

C002 — A `CNNN` reference in any ADR or card is always scoped to the
specific ADR it names (`ADR-005C002`), never bare (`C002` alone).

## Alternatives considered

- **Share the §NNN sequence.** Simple, but a new commitment inserted between
  §003 and §004 would require renumbering — breaking every citation of §004+.
  Rejected.

- **`§C` two-character prefix.** Proposed initially; rejected during review
  in favour of the single character `C` for consistency with the low-friction
  principle.

- **Leave "Committed to" as prose.** Sufficient for short ADRs; breaks down
  when you need to supersede one invariant without touching the others.
  Rejected.

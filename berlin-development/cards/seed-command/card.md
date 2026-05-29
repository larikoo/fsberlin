---
id: 01JBSEEDCOMMAND00000000000
title: "berlin seed: low-friction card capture"
type: card
building_status: pending
priority: medium
assignee: claude-code
skills: [rust]
depends_on: [p1-fs-walker, p1-frontmatter-parser, p1-cli-init, adr-012-atomic-decision-clauses]
created: 2026-05-29
---

`berlin seed <type> "<title>"` scaffolds a new, valid card folder so a human
(or agent) can drop an idea in fast and let the substrate handle the
boilerplate. The capture end of the same low-friction philosophy as the
two-phase triage drip.

Filed post-Phase-1 (out of Phase 1's locked scope); unphased backlog. Small —
it reuses the walker, typed models, and init's ULID/date helpers.

## Scope: mechanical consistency only (ADR-006)

Seed owns **deterministic scaffolding**, never semantic judgement.

- ✅ Generates: ULID `id`; today's `created`; correct `type`; the type's
  default status (`planning_status: proposed` / `building_status: pending`);
  the **next sequential `adr_number`** (walk + parse existing ADRs, max + 1);
  a unique slug derived from the title; and the right body skeleton — for
  ADRs the Nygard template pre-stubbed with `§NNN` clause placeholders
  (ADR-012) plus a `why.md` stub.
- ❌ Does NOT judge whether the idea conflicts with another card or is sound —
  that is Spymaster (advisory) + human review (ADR-006). Seed scaffolds
  structure; it never edits meaning.

## Deliverable
- `berlin seed adr "<title>"` and `berlin seed card "<title>" [--phase N]`
  (waypoint/phase later if useful).
- Errors on slug collision; refuses to overwrite an existing card.
- Optional `--dry-run` prints the would-be path + frontmatter without writing.

## Done when
- `berlin seed adr "X"` creates a folder that `berlin validate` accepts with
  no findings, with `adr_number` = current max + 1 and `planning_status:
  proposed`.
- `berlin seed card "Y"` likewise (`building_status: pending`).
- A title colliding with an existing slug is rejected with a clear message.
- Generated ADR body carries empty `§001…` clause stubs per ADR-012.

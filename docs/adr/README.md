# Architecture Decision Records

This directory contains the ADRs for FSBerlin. Each ADR documents one
architectural commitment.

## Format

Each ADR follows the Michael Nygard template:

```markdown
# ADR-NNN: Short title

Date: YYYY-MM-DD

## Context
What's the situation, what forces are in play.

## Decision
What we're doing. Stated as a commitment.

## Consequences
What becomes easier; what becomes harder; what we're committed to.

## Alternatives considered
What else we looked at; why those were rejected.
```

The `Status:` line from the original Nygard template is **not used**.
Status is tracked in the card frontmatter as `planning_status:` (see
`schema/card.schema.yaml` and ADR-009). The body is append-only prose;
the frontmatter is the machine-readable source of truth.

## Conventions

- **One ADR per architectural commitment.** Don't combine unrelated
  decisions. ADRs should be supersedeable independently.
- **Numbered sequentially.** ADR-001, ADR-002, ... Never reused.
- **Append-only.** ADRs don't get edited after acceptance, they get
  superseded. The new ADR explicitly states which one it supersedes
  and why.
- **Stored as cards.** Each ADR is also a card in `berlin-development/`,
  with its own status, why.md, and audit trail.
- **One page is usually enough.** If an ADR is longer than two pages,
  ask whether it's really one decision.

## Decision clauses (§NNN) — see ADR-012

Every normative statement in the **Decision** section carries a marker
`§NNN`, unique within the ADR, assigned in order, and **never reused or
renumbered**. A clause is one independently-acceptable claim — if you can
accept half of it, it's two clauses. Numbering is flat (`§007`, never
`§004.1`); only the Decision section is numbered (Context, Consequences, and
Alternatives stay prose).

- **Reference** a clause as `ADR-002§002`. The validator resolves it; a
  dangling reference is a blocking error.
- **Supersede** at clause granularity: a later ADR sets
  `supersedes: [adr-002§002]` and replaces just that clause. The superseded
  clause is marked, not deleted.
- **Review** serves clauses one at a time; answer `y`/`n` and the verdict is
  appended to the review record against its `§ref` without further discussion
  (`berlin review <adr>`; Phase 4 MCP tool).

> ⚠️ **Disclaimer: do not dev and drive.** The one-clause-at-a-time review
> flow is built to be usable from a phone — which makes it tempting in the
> wrong places. It is mobile-friendly, not road-friendly. Triage from a safe
> stop; the clauses will still be there when you've parked.

## Status meanings

Status is the `planning_status:` field in the card frontmatter:

- **`proposed`:** drafted, under review, not yet binding.
- **`in-discussion`:** actively being debated; may still change.
- **`accepted`:** in force. Implementations must respect it.
- **`superseded`:** historical. Set `superseded_by:` to the new ADR
  slug. The reasoning may still be educational, but the superseding
  ADR is now authoritative.
- **`withdrawn`:** abandoned without a replacement.

## Why ADRs matter here

FSBerlin's architectural choices are non-obvious. "Card = folder" looks
naive without the four-layer argument. "Agents are users; models are
runtime" looks like over-engineering without the persistent-identity
argument. Anyone reading the codebase cold will reach for defaults that
contradict the architecture. The ADRs defend the non-obvious choices
from drift.

They are also the load-bearing prompt for Claude Code and other AI
agents working in this repo. See `CLAUDE.md` at the repo root.

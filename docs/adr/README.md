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

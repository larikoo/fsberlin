# Architecture Decision Records

This directory contains the ADRs for FSBerlin. Each ADR documents one
architectural commitment.

## Format

Each ADR follows the Michael Nygard template:

```markdown
# ADR-NNN: Short title

Date: YYYY-MM-DD
Status: Proposed | Accepted | Superseded by ADR-MMM

## Context
What's the situation, what forces are in play.

## Decision
What we're doing. Stated as a commitment.

## Consequences
What becomes easier; what becomes harder; what we're committed to.

## Alternatives considered
What else we looked at; why those were rejected.
```

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

- **Proposed:** drafted, under review, not yet binding.
- **Accepted:** in force. Implementations must respect it.
- **Superseded by ADR-N:** historical. The reasoning may still be
  educational, but ADR-N is now authoritative.

## Why ADRs matter here

FSBerlin's architectural choices are non-obvious. "Card = folder" looks
naive without the four-layer argument. "Agents are users; models are
runtime" looks like over-engineering without the persistent-identity
argument. Anyone reading the codebase cold will reach for defaults that
contradict the architecture. The ADRs defend the non-obvious choices
from drift.

They are also the load-bearing prompt for Claude Code and other AI
agents working in this repo. See `CLAUDE.md` at the repo root.

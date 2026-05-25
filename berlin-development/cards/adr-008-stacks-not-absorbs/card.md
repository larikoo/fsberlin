---
id: 01JBADR008000000000000000
title: "Stack with adjacent tools; do not absorb"
type: adr
adr_number: 8
planning_status: accepted
priority: high
phase: 0
assignee: lari
skills: [architecture, design]
depends_on: []
created: 2026-05-25
---

See body in this folder. Review with the human before promoting to
`planning_status: accepted`.

The decision text below is a draft based on the May 2026 design
conversation. It needs your eye before lock-in.

---


# ADR-008: Stack with adjacent tools; do not absorb

Date: 2026-05-25

## Context

Beads (Steve Yegge's git-backed issue tracker for AI coding agents)
overlaps with FSBerlin: both persist project state in git, both
provide agent-readable structured data, both have "tasks with
dependencies." The natural temptation is either to absorb Beads's
functionality into FSBerlin, or to integrate so tightly that FSBerlin
depends on Beads.

Both are wrong moves. Beads is focused on a different layer (L2 agent
memory for coding work, JSONL-optimized for machine reads). FSBerlin
is a broader substrate (waypoints, mind maps, authority gradients,
agent definitions, why-as-first-class-field). Each excels at its
scope. Forcing them into one tool loses value on both sides.

The same applies to git itself, to `.editorconfig`, to language
toolchains, and to any tool that drops state into the project
directory.

## Decision

FSBerlin stacks alongside adjacent tools. It does not absorb them.

Concretely:

- FSBerlin treats `.beads/`, `.git/`, `.github/`, `.idea/`,
  `node_modules/`, `target/`, and similar tool-owned directories as
  opaque. No scanning, no indexing, no agent reads through FSBerlin.
- Cross-references happen at the human level. An FSBerlin card may
  mention `bd-42` in its description; that's curation, not
  integration.
- FSBerlin agents (Spymaster, Sentinel) do not have read access to
  opaque directories. Their scope excludes them by default.

The opacity list is configurable per-project but starts with sensible
defaults including all the above.

**Beads as AI session memory.** AI agents operating as users may use
the `bd` CLI to write session context — notes that survive across
conversation sessions, track open questions, or record decisions made
mid-session. This is the AI *using Beads as a tool*, the same way a
human would. It does not violate the opacity rule: FSBerlin-the-substrate
never reads `.beads/`; the AI agent is simply running `bd` commands
outside the substrate's scope. The session memory pattern is encouraged
— it gives AI agents continuity without burdening the card system with
ephemeral context.

## Consequences

**Easier:**
- Migration into FSBerlin is low-cost; existing tools keep working.
- FSBerlin's scope stays bounded; doesn't reinvent issue tracking.
- Adjacent tools' contributors don't need to coordinate with
  FSBerlin's.
- Composition via human curation; the human decides what crosses the
  boundary.

**Harder:**
- Cross-tool searches require the human to know about both tools.
- Some redundancy: FSBerlin cards and Beads issues may describe
  overlapping work.

**Committed to:**
- The opacity list is part of the spec.
- FSBerlin will never offer "Beads integration" that reads .beads/.
- Other "adjacent tool" support (a future Linear connector, for
  example) follows the same pattern: opaque by default, human-curated
  cross-reference.

## Alternatives considered

- **Absorb Beads.** Reinvents 18K+ stars of focused work. Loses
  Beads's specific design choices that work for coding agents.

- **Tight integration (read .beads/).** Couples FSBerlin to Beads's
  internal format. Yegge himself describes Beads as "a crummy
  architecture (by pre-AI standards) that requires AI to handle all
  its edge cases." Coupling to that is a long-term liability.

- **Mutual exclusion (require choosing one).** Forces a choice that
  doesn't need making. Users benefit from both.

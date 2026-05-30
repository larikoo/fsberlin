---
id: 01JBADR016000000000000000
title: "Spymaster surfaces findings proactively (severity-gated digest)"
type: adr
adr_number: 16
planning_status: accepted
priority: medium
assignee: lari
skills: [architecture, design]
depends_on: []
linked: [adr-006-validators-vs-spymaster, adr-014-project-files-cold-reading]
supersedes: [adr-006§003]
created: 2026-05-30
---

Drafted and accepted 2026-05-30 (berlin-v4b) via a three-clause drip. Amends
the advisory layer of ADR-006: the passive "humans promote findings" model
gains a proactive surfacing channel. Authority is unchanged — delivery only.

---


# ADR-016: Spymaster surfaces findings proactively (severity-gated digest)

Date: 2026-05-30

## Context

ADR-006 §003 made the advisory layer (Spymaster, Sentinel) **pull-only**:
findings are written to `findings/spymaster/` and `findings/sentinel/`, and a
human goes looking and promotes them. In practice an unread finding is wasted
situational awareness — the most valuable thing Spymaster catches ("the things
you're too close to see") is worthless if it sits in a folder no one opens.

This is sharpened by ADR-014, which gives Spymaster a standing job:
watching `context.md` for drift, staleness, and reached-waypoint
reconciliation. That job only pays off if the findings reach the human while
they still matter.

The risk to manage: proactive surfacing must not become noise (banner
blindness, the exact failure ADR-005 rejected for per-action prompts) and must
not interrupt in-flight work or break the single-keystroke response economy.

## Decision

§001 — Spymaster and Sentinel **surface findings proactively** in the human's
working channel, not only by writing to `findings/`. The advisory layer is
permitted to speak, not just file. (Amends ADR-006 §003, which was pull-only.)

§002 — Announcements are an **end-of-turn digest, never a mid-work
interruption**: findings surface at a natural pause, after the current action
completes, batched. They obey the single-keystroke economy — a finding is a
one-line headline the human acts on or waves past, not a wall to scroll.

§003 — Proactive surfacing changes **delivery only, not authority**. Spymaster
stays read-only; every finding is still written to `findings/` as the system
of record; the human still promotes any finding to action (ADR-005/006
unchanged). **Severity-gated:** routine findings live in `findings/` and
surface only on query; only high-severity findings — security drift, authority
drift, context↔reality conflicts — surface proactively in the digest. The bar
is "you'd want to know now," not "everything I noticed."

## Consequences

**Easier:**
- The situational-awareness Spymaster exists for actually reaches the human
  in time to matter (e.g. "context.md contradicts a card you just changed").
- The single-keystroke economy is preserved — a digest of headlines, not a
  dump of every observation.

**Harder:**
- Severity classification is now load-bearing: Spymaster must rank findings,
  not just produce them. Mis-ranking either spams (false high) or buries
  (false low).
- The surfacing channel is implementation-specific (CLI, MCP, future UI);
  each must honor the digest + severity contract.

**Committed to:**

C001 — Proactive surfacing never changes authority: read-only, findings-as-
record, human-promoted (ADR-005/006 hold).

C002 — Routine findings never interrupt; only high-severity findings surface
proactively, and always as an end-of-turn digest, never mid-action.

## Alternatives considered

- **Keep pull-only (ADR-006 §003 as-is).** Simple, but the most valuable
  findings die unread. The whole point of an advisory layer is undermined.
  Rejected.

- **Surface everything proactively.** Banner blindness — the ADR-005 failure
  mode. The human learns to ignore the channel. Rejected in favour of
  severity gating.

- **Immediate (mid-work) surfacing for high-severity.** Considered; rejected
  because interrupting an in-flight operation is worse than a few seconds'
  delay to the next pause, and it breaks the keystroke flow. End-of-turn
  digest is the cadence even for high-severity.

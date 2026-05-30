---
id: 01JBADR018000000000000000
title: "Agent ↔ substrate orchestration: autonomy bounded by floors"
type: adr
adr_number: 18
planning_status: in-discussion
priority: high
assignee: lari
skills: [architecture, design]
depends_on: []
linked: [adr-004-agents-vs-models, adr-005-hitl-floors, adr-006-validators-vs-spymaster, adr-007-editor-as-peer, adr-008-stacks-not-absorbs, adr-014-project-files-cold-reading, adr-016-spymaster-proactive]
created: 2026-05-30
---

IN-DISCUSSION — §001–§007 accepted via drip 2026-05-30 (resolving berlin-apy);
§008 is the one open decision, deferred to next session (medium-agnostic vs
code-only). Do not promote to accepted until §008 is decided.

The big one: how agentic coders (and other agents) and the FSBerlin substrate
work together. Phase 4 (MCP) is where most of this is implemented.

---


# ADR-018: Agent ↔ substrate orchestration

Date: 2026-05-30

## Context

berlin-apy asked how agents and the substrate walk hand in hand: who
dispatches, how card state and work state stay in sync, push vs pull,
CLAUDE.md vs dynamic context, where the PR loop lives, and how multiple agents
coordinate. The drip below resolved it — except §008, which surfaced a real
scope question (is this software-dev-only, or medium-agnostic?) deferred to
the next session.

The governing correction made during the drip: the substrate is **not**
predicated on human supervision of every action. Per ADR-005 the human is a
gate at exactly two floors (invariant, external-effect); between them, agents
work freely. Autonomy is the design intent, bounded by the floors — not by
requiring a human to initiate each step.

## Decision

§001 — **Orchestration may be autonomous.** A scheduler/dispatcher agent that
finds ready cards and claims/assigns them on a schedule is permitted — an
ordinary agent bounded by its `write_scope` and the HITL floors (ADR-005), not
a new trust root. The constraint on autonomy is the two floors, not human
initiation: an autonomous loop may dispatch → claim → build → commit → open PR
freely; it cannot merge/publish (external-effect floor) or change invariants
(invariant floor), because those require a human signature and no AI may sign
(ADR-005 C002). Pull is the default; push is fully supported. The floors
contain both.

§002 — **The claim is the coordination primitive.** An agent takes a card by
setting `building_status: in-progress` plus a claim record (claiming agent id
+ work branch). A claimed card is not taken twice. Claims can be released or
expire.

§003 — A claimed card carries a **`work_branch`** field linking card state to
the git branch where code accrues (attributed per ADR-004). One card, one
active work branch. *(NOTE: §008-pending may generalize `work_branch` →
`work_ref` for non-code media.)*

§004 — **Completion authority is declared per-card, not fixed.** The card spec
names who decides "done": `self` (working agent), `agent`/daemon (a validation
pass), or `human`. `propose_done` is the universal "I believe this is
complete" signal; what follows is the declared authority. The floors are the
immovable override: if reaching `done` crosses an external-effect line, that
step needs a human signature regardless. A card whose completion crosses no
floor can be self-closed or validated-closed autonomously. (Assumption: an
agent reports failure cleanly on its own — no elaborate "authority
unavailable" fallback machinery needed.)

§005 — **Context is delivered as a bundle of distilled essentials, with
pointers to the deep tiers.** On claim/open, the MCP server resolves the
card's already-curated structure into one window: the card, its `depends_on`
ADRs, phase context, the project `context.md`, severity-relevant findings
(ADR-016), and the `important/`-tier skill orientation for the card's
`skills:`. Resolution, not guesswork — `depends_on`, distillation, and tiering
already did the curation (ADR-014/015). The deep material (`reference/`, full
skill content, raw sources) is not pre-loaded; it is reachable on demand via
pointers the bundle carries.

§006 — **Three context layers, vendor-neutral at the base.** `AGENTS.md` is
the canonical, model-ignorant cold-boot (required reading, hard rules);
vendor-specific files (`CLAUDE.md`, `.cursorrules`, …) are thin adapters that
defer to it and carry only tool-specific bits (ADR-008), never canonical.
`agent.yaml` is identity/scope/model/schedule (ADR-004). The
`get_card_with_context` bundle (§005) is the warm per-card context. Bootstrap
→ identity → task.

§007 — **Multi-agent coordination needs no lock manager.** It falls out of
three already-decided things: claims (§002) partition work; `write_scope`
(ADR-004) partitions files; git's last-known-SHA check (ADR-007) catches any
write collision. A read-only Spymaster running alongside a builder is safe by
construction.

§008 — **[OPEN — deferred to next session.]** Is the substrate medium-agnostic
or software-dev-centric? Two options on the table:

- **(A, recommended) Medium-agnostic.** The core (cards/phases/claims/
  completion authority/floors) is medium-neutral; code is one *adapter*.
  Generalize `work_branch` → `work_ref` (branch | path | URL). The "done"
  event and external-effect floor are medium-specific (code merge / article
  publish / video export / dissertation submit), all routed through §004.
  FSBerlin reinvents no output tool (ADR-008); the git/forge PR loop is the
  *code adapter* — first to build, not the only one. Rationale: the core is
  already neutral (we published to Substack today — floor was "publish," not
  merge); `why.md` calls it a "project lab substrate," not a code tool; cost
  of generality is near-zero.
- **(B) Code-only.** Accept ≈99% software-dev usage; hardcode the git/PR
  model; `work_branch`/merge stay code-specific; don't generalize.

Lari leaning A (wants to run a dissertation on FSBerlin) but reserved the
call. Decide §008, then promote this ADR to accepted.

## Consequences

*(Finalize when §008 is decided — Easier/Harder/Committed-to depend on A vs B.)*

Provisional Committed-to (independent of §008):

C001 — Autonomy is bounded by the two floors (ADR-005), not by human
initiation. No AI may sign; the floors are the immovable line.

C002 — Coordination uses claims + `write_scope` + git's SHA check; the
substrate adds no new concurrency primitive.

C003 — `AGENTS.md` is the canonical, vendor-neutral agent bootstrap; vendor
files are adapters, never canonical.

## Alternatives considered

- **Push-only orchestrator with dispatch authority.** Rejected in §001: a
  scheduler is just a scoped agent, not a new trust root; mandating a standing
  orchestrator adds a failure mode for a file-and-git system. Both push and
  pull are permitted instead.
- **Agent self-marks `done`.** Rejected in §004: completion authority is
  declared per-card and the floors override; an agent cannot self-cross an
  external-effect line.
- **Context as a manifest of pointers (no pre-loaded content).** Rejected in
  §005: pushes curation back onto the agent at read-time — the exact work the
  substrate already did via `depends_on`/distillation/tiering. Bundle the
  distilled essentials; point at the deep tiers.
- **CLAUDE.md as canonical bootstrap.** Rejected in §006: vendor-specific.
  `AGENTS.md` (neutral standard) is canonical; CLAUDE.md is an adapter.

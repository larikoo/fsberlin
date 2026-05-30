---
id: 01JBADR014000000000000000
title: "Project root is ordered for cold reading; context.md replaces phantom files"
type: adr
adr_number: 14
planning_status: in-discussion
priority: medium
assignee: lari
skills: [architecture, design]
depends_on: []
linked: [adr-001-filesystem-as-substrate, adr-003-waypoints-as-overlays, adr-005-hitl-floors]
created: 2026-05-30
---

Promoted to in-discussion 2026-05-30 after the context-distillation design
conversation. ADR-015 (context distillation system) depends on this one;
ADR-014 defines context.md as the invariant-floor orientation file that
ADR-015's Distiller produces.

Four goals drove this ADR: readability (what to read first comes first),
simplicity (one place per concept), context economy (help AI keep its window
small), and flexibility (a messy folder of project files doesn't break
anything). Design principle: the folder structure teaches you what to read and
in what order, without the berlin CLI.

---


# ADR-014: Project root is ordered for cold reading; context.md replaces phantom files

Date: 2026-05-30

## Context

The SPEC §2.1 layout lists `requirements.md` and `skills.md` at the project
root alongside `why.md`. Neither file is created by `berlin init`, neither
has a schema, neither is mentioned in any ADR. They are phantom files —
present in the spec, absent everywhere else.

More broadly, the current layout has no implied reading order. A cold reader
(human or AI agent arriving mid-project with no prior context) sees:

```
why.md
.fsberlin/
cards/
agents/
waypoints/
findings/
```

Nothing in this structure signals: start here, then here. The substrate
claims to be self-explanatory without tooling (ADR-001), but the layout
doesn't fulfill that claim. An agent loading context has no guidance on which
files matter most — it either loads everything (expensive) or guesses.

A related question: `why.md` is currently hardcoded as both an orientation
file and an invariant-floor sentinel (render guard, validator, init). If the
identity document could be named differently (e.g. `why.html` for a rendered
view), the invariant-floor concept needs to generalise from a filename to a
role. This is deferred but flagged.

Inspiration: ICM (Interpretable Context Methodology) uses numbered folders
and per-stage `CONTEXT.md` files to impose reading order and constrain the
agent's context window to only what is needed at each step. The same
efficiency applies here at the project level.

## Decision

§001 — The project root is ordered for cold reading. The convention, in
reading order, is:

```
why.md          ← 1. why this exists (unchanged; invariant floor)
context.md      ← 2. what's happening now + constraints + skills in play
cards/          ← 3. the work
waypoints/      ← 4. where we're aiming
agents/         ← 5. who's working
.fsberlin/      ← 6. tooling config (hidden by default; last)
findings/       ← (advisory output; read when relevant)
```

Any other files and folders at the root (sources, working files, output,
imported documents) are ignored by FSBerlin (ADR-001/008) and do not break
anything. The layout does not own the whole tree.

§002 — `requirements.md` and `skills.md` are retired as named files. They
are replaced by `context.md` — a single prose orientation file, no schema,
no frontmatter. Content:

- **Current state:** one paragraph on where the project is right now.
- **Constraints:** the non-negotiables that govern decisions (what
  `requirements.md` would have held).
- **Skills in play:** what the skill tags used in this project mean and
  which agents carry them (what `skills.md` would have held).
- **What to do next:** optional — the ICM-style "read this before touching
  any cards" instruction to an incoming agent.

§003 — `context.md` is a **living document**, human-maintained, not
schema-governed. It is not a card. It does not have frontmatter. It is prose
that a human or agent can read in 60 seconds and be oriented. Agents should
read it before loading any cards.

§004 — `context.md` is an **invariant-floor file** alongside `why.md`:
waypoint overlays may not shadow it, and changes require human authorship
(same treatment as `why.md`). Rationale: it holds the project's current
constraints — overriding it in a milestone projection would let a waypoint
silently change what the project is allowed to do.

§005 — `berlin init` creates a stub `context.md` alongside `why.md`. The
stub prompts for the four sections above. An empty `context.md` is valid
(validates clean); a missing one is a warning, not a blocking error.

§006 — The **invariant-floor concept generalises from filename to role**:
the floor is defined as files with a specific *role* in the project, not
specific *names*. For now the canonical names (`why.md`, `context.md`,
`schema/*`) remain the implementation — but the principle is role-based,
enabling a future ADR to allow `why.html` or similar as an alternative
rendering of the same invariant content (with `why.md` as the source of
truth). Phase 9 (view renderer) is the natural home for rendered outputs.

## Consequences

**Easier:**
- A cold reader — human or AI — knows exactly what to read and in what
  order without a README or the berlin CLI.
- Context economy: an agent oriented by `why.md` + `context.md` (two short
  files) before loading any cards keeps its window small and focused.
- One place per concept: constraints and skill definitions live in
  `context.md`, not scattered across `requirements.md`, `skills.md`, and
  card descriptions.
- Flexibility: any other files at the project root are ignored by FSBerlin;
  a messy folder of sources/working files/outputs doesn't break validation.
- The invariant-floor generalisation opens the door for rendered variants
  (`why.html`) as Phase 9 outputs without changing the architecture.

**Harder:**
- Existing projects have no `context.md`; a migration/creation step is
  needed (low-friction: `berlin init` in an existing project could offer
  to create it, or users create it by hand).
- `context.md` is prose — no machine-checkable schema. Its quality is
  the human's responsibility. Spymaster could flag a stale `context.md`
  (e.g. "context.md hasn't been updated in 30 days but 12 cards have
  changed") but cannot validate its content.
- The render guard and validator must be updated to treat `context.md`
  as an invariant-floor file alongside `why.md`.

**Committed to:**

C001 — `why.md` remains the primary identity document and invariant-floor
sentinel. `context.md` is a second invariant-floor file added by this ADR.

C002 — `requirements.md` and `skills.md` are not created by `berlin init`
and not referenced in the spec after this ADR. Any existing files by those
names in a project are user files; FSBerlin ignores them.

C003 — `context.md` has no frontmatter and no schema. It is prose. Adding
schema to it is a future ADR, not a silent extension.

C004 — The invariant-floor concept is role-based, not filename-based, as a
principle. The canonical filenames are the current implementation; changing
them requires an ADR.

## Alternatives considered

- **Keep `requirements.md` + `skills.md` as separate files.** Violates
  the simplicity goal (two places for related orientation content) and
  doubles the files a cold agent must load. Rejected.

- **Fold everything into `why.md`.** Simpler, but conflates stable identity
  ("why does this exist, forever") with current state ("what's happening
  now, this week"). The two have different update cadences and different
  authors (why.md changes rarely; context.md changes often). Rejected.

- **Numbered root files (01_why.md, 02_context.md).** ICM-style. Explicit
  reading order, but adds friction when referencing files and looks
  unusual in a project repo. Naming convention achieves the same goal
  more naturally. Rejected.

- **`context.md` as a schema-governed card.** Would enable machine
  validation but defeats the "60-second prose orientation" goal and adds
  ceremony to the most human-facing file in the project. Rejected.

- **Allow `why.html` now.** The rendered-view use case is real but belongs
  in Phase 9 (view renderer). Generalising the invariant-floor concept to
  roles (§006) is the right preparation; allowing alternative formats now
  adds complexity without Phase 9's rendering infrastructure. Deferred.

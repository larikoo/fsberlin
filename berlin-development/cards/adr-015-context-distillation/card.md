---
id: 01JBADR015000000000000000
title: "Context distillation: context/ folder, S-numbered sources, berlin distil"
type: adr
adr_number: 15
planning_status: accepted
priority: medium
assignee: lari
skills: [architecture, design]
depends_on: [adr-014-project-files-cold-reading]
linked: [adr-001-filesystem-as-substrate, adr-004-agents-vs-models, adr-005-hitl-floors, adr-008-stacks-not-absorbs]
created: 2026-05-30
---

Drafted and accepted 2026-05-30 via a nine-clause triage drip. Defines the
full context distillation system: the context/ folder hierarchy, S-numbered
source clippings, the berlin distil verb, and the context.md lifecycle.
Depends on ADR-014 (which defines context.md as an invariant-floor
orientation file). Also establishes the principle that named agents are for
persistent recurring roles — one-shot operations are verbs.

---


# ADR-015: Context distillation — context/ folder, S-numbered sources, berlin distil

Date: 2026-05-30

## Context

ADR-014 established `context.md` as the project root's orientation file — the
60-second read that orients any cold reader (human or AI) before they touch a
card. But it left open: where does the content of `context.md` come from, and
how does raw source material (PDFs, briefs, transcripts, highlighted documents)
relate to it?

The answer is a distillation process. The raw material is a dump; the
substrate is mined from it; future use is a breeze because the mining was done
once. This ADR defines that process, its data model, and its tooling.

The design is governed by four principles from ADR-014: readability (what to
read first comes first), simplicity (one place per concept), context economy
(help AI keep its window small), and flexibility (a messy dump doesn't break
anything). A fifth principle added here: **paranoia is a virtue dealing with
agents** — when in doubt, restrict access rather than grant it.

## Decision

§001 — The `context/` folder at the project root is the raw intake for
project context material: PDFs, briefs, transcripts, highlighted documents,
images, any format. FSBerlin treats it as opaque by default (ADR-008): no
scanning, no indexing, no agent reads it unless explicitly in scope. It is
the dump; it does not need to be tidy to be useful.

§002 — The Distiller classifies source files into three tiers, physically
moving them into subfolders:
- `context/important/` — high-frequency; agents may read freely.
- `context/reference/` — medium-frequency; agents read on demand, with
  explicit human-granted scope per invocation. The decision to grant
  `reference/` access is situational: a simple model on a bounded task may
  be given it freely; a complex reasoning task with a constrained window may
  not. Access policy is a per-invocation parameter, independent of the tier.
- `context/garbage/` — kept as "just in case"; agents are never granted
  access without direct human instruction.

Tiers are a property of the source material, set at distillation time but
not frozen. **Promotion or demotion of a file between tiers at any time is
valid; the system adapts.** Access policy follows the file's current tier,
not its original classification.

§003 — Each source is a **specific extracted point** — a text clipping,
highlighted passage, or cropped image — saved as its own file named
`S00001-slug.ext` (five-digit namespace: `S00001`…`S99999`). Reference
numbers are per-project, sequential, never reused — same discipline as
`§NNN` decision clauses and `CNNN` commitments. No separate index file:
the filename convention is the index. Citations in `context.md` use
`[S00001]`; `rg` works directly on the clipping files.

§004 — `berlin distil` is a **verb, not a named agent**. It takes the
current dump, calls whatever model is configured at invocation time (runtime
parameter, ADR-004), and produces the draft. Named agents are for persistent,
recurring roles with track records (ADR-004); one-shot and periodic operations
are verbs. This is a general principle: do not create a named agent for an
operation that has no continuity to preserve.

§005 — The draft produced by `berlin distil` goes directly to `context.md`
at the project root and clipping files into `context/important/`,
`context/reference/`, `context/garbage/`. No staging folder. `context.md`
carries minimal frontmatter marking it as `status: draft`. The human reviews,
edits freely, and promotes by flipping `status: reviewed`. `berlin distil`
is idempotent — re-running updates the draft without moving anything already
classified. A human who chooses to proceed with `status: draft` may do so;
the system notes it but never blocks on it.

§006 — `context.md` carries **minimal frontmatter** — not a card schema:

```yaml
---
distilled: 2026-05-30
distilled_by: claude-opus-4-8   # or "human"
status: draft | reviewed
reviewed: 2026-06-01            # optional, set when human accepts
---
```

A `draft` context.md is valid and useful; a missing context.md is a warning;
a `draft` context.md older than a configurable threshold is a Spymaster
finding. The validator never blocks on context.md status — advisory only.

§007 — **Principle: data mining first, future use is a breeze.** The value
of `berlin distil` is not just `context.md` — it is the classified,
numbered, browsable `context/` folder that results. A project whose context
is mined once is easier to work with for every subsequent agent and human
session: the important material is findable in seconds, the window stays
small, the garbage is out of the way but not lost.

Re-distillation is **additive**: existing `S`-numbered files keep their
numbers and tier assignments unless the human explicitly re-classifies them.
`berlin distil` only processes unclassified files in the raw dump — it does
not rebuild the whole `context/` folder. Spymaster flags new unclassified
files; the human decides when to run `berlin distil` again.

§008 — `berlin distil` is a single verb with no persistent agent identity.
It takes the current unclassified dump, calls the configured model, and
produces the draft. This is the correct abstraction for a one-time or
periodic operation. The general principle applies: do not create a named
agent for an operation that has no continuity to preserve.

§009 — The **cold-reading order** established in ADR-014 is updated:

```
why.md          ← 1. why this exists (invariant floor)
context.md      ← 2. current state + constraints + cited sources
context/        ← 3. source material
  important/    ←    freely readable by agents
  reference/    ←    on demand, per-invocation grant
                     (garbage/ exists but is not advertised here;
                      agents have no access by default)
cards/          ← 4. the work
waypoints/      ← 5. where we're aiming
agents/         ← 6. who's working
.fsberlin/      ← 7. tooling config
findings/       ←    advisory output (read when relevant)
```

`context/garbage/` is present on disk but excluded from the cold-reading
order. Agents do not know it exists unless a human explicitly grants access.
Paranoia is a virtue when dealing with agents.

## Consequences

**Easier:**
- Context economy: an agent oriented by `why.md` + `context.md` +
  `context/important/` keeps its window small and focused, with a clear
  path to `reference/` when the task warrants it.
- Source provenance: every claim in `context.md` traces to a numbered
  clipping file; citations are verifiable with `rg [S00012] context/`.
- Re-distillation is safe: additive numbering means no existing citation
  breaks when new sources are added.
- Promotion/demotion between tiers is frictionless — move the file,
  access policy follows automatically.

**Harder:**
- `berlin distil` is a new verb (Phase implementation TBD — likely Phase 7
  alongside Spymaster/Sentinel, which share the same advisory-layer pattern).
- Spymaster needs a new finding type: "unclassified files in context/ dump."
- The validator gains a new check: `[S00001]` citations in `context.md`
  must resolve to files in `context/`.

**Committed to:**

C001 — Source numbers (`S00001`…) are immutable and never reused, same
discipline as `§NNN` and `CNNN`.

C002 — `berlin distil` is a verb, not a named agent. This principle
generalises: one-shot and periodic operations are verbs; agents are for
persistent recurring roles.

C003 — `context/garbage/` is never advertised in the cold-reading order
and never in any agent's default `read_scope`.

C004 — Re-distillation is always additive. `berlin distil` never
renumbers or reclassifies existing sources without explicit human
instruction.

C005 — The validator advises on context.md status; it never blocks.
The human's choice to proceed with an unreviewed context is valid.

## Alternatives considered

- **Named Distiller agent.** Rejected: distillation has no continuity to
  preserve across runs; a named agent would be an identity without a track
  record. Verbs are the right abstraction for one-shot operations (§004/§008).

- **sources.yaml index with embedded clippings.** Rejected: embeds content
  in a data file, defeating grep-ability and the "filesystem is the interface"
  principle. The filename convention is the index; the file is the content.

- **Three-character S-prefix (S001).** Rejected: a large research project
  or long-running codebase hits 999 easily. Five digits (S00001) gives
  99,999 sources before rollover and keeps filesystem sort order clean.

- **Draft staging folder (findings/distiller/).** Rejected: a staging folder
  is a database in disguise. Status frontmatter in context.md is simpler,
  more readable, and consistent with how the rest of the system tracks
  lifecycle state.

- **Access policy baked into tier definition.** Rejected: tier is a property
  of the source material (stable); access policy is a property of the agent
  invocation (situational). Conflating them removes the flexibility to grant
  reference/ access for a specific task without reclassifying the files.

- **Advertising garbage/ in cold-reading order.** Rejected: paranoia is a
  virtue when dealing with agents. Files agents shouldn't read by default
  should not be in their line of sight.

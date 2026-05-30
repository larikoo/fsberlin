---
id: 01JBADR017000000000000000
title: "Card security posture: no secrets, repo-scoped visibility, scope-governed access"
type: adr
adr_number: 17
planning_status: accepted
priority: medium
assignee: lari
skills: [architecture, design]
depends_on: []
linked: [adr-002-card-equals-folder, adr-004-agents-vs-models, adr-005-hitl-floors, adr-006-validators-vs-spymaster, adr-008-stacks-not-absorbs]
created: 2026-05-30
---

Drafted and accepted 2026-05-30 (berlin-abo) via a four-clause drip. States
the card-data security/privacy posture flagged during the ADR-002 review.
Establishes the default stance; the Phase 5 authority/HITL design may refine
specifics but inherits this default.

---


# ADR-017: Card security posture

Date: 2026-05-30

## Context

Flagged during the ADR-002 review (berlin-abo): cards contain user data —
assignee, relations, free-form content — and live in a git repository that may
be shared or public. Four questions were parked: what is sensitive in a card,
whether any card types/fields need access control beyond the HITL floors,
whether cards need a visibility/scope field, and the privacy implications of a
shared repo.

This ADR answers them as a posture. It is deliberately minimal — the bet is
that card security is mostly *not* a card-level feature, but a consequence of
existing mechanisms (opaque paths, agent scope, the HITL floors, secret
scanning) plus human judgement at authorship. Phase 5 (authority/HITL) may
refine the specifics; it inherits this default.

## Decision

§001 — **Cards are not a secrets store.** Tokens, credentials, and data that
must not enter version control do not belong in card content or frontmatter.
Secrets live in `.env` (gitignored) or opaque paths (ADR-008); the
secret-scanning validator (ADR-006) blocks commits that leak them. A card that
needs to reference a secret references its location, never its value.

§002 — **Card visibility is the repository's visibility — not a per-card
property.** A card is exactly as public or private as the git repo it lives
in. There is no per-card `visibility`/`scope` field through Phase 5.
Finer-grained visibility, if ever needed, is a real multi-tenant requirement
deferred to a future ADR — not built speculatively now.

§003 — **Access control is governed by agent `write_scope` (ADR-004) and the
HITL floors (ADR-005), not by per-card ACLs.** Access is a property of the
agent, not the card. A card carries no permission fields; an agent's scope
glob plus the invariant and external-effect floors decide what it may touch.
Authority stays in one place — the agent definition — not scattered across
every card.

§004 — **The substrate makes leaks catchable, not impossible.** FSBerlin does
not promise that a card in a repo the human chose to publish stays private; it
promises the tools to catch what shouldn't ship: secret scanning, opaque
paths, the pre-push check. The human is the gate at authorship; the
secret-scanning floor is the safety net. (Lived example: a personal email
reached the issue-tracker data and was caught before the public push, not
prevented by the substrate.)

## Consequences

**Easier:**
- No new card-level machinery: security falls out of opaque paths, agent
  scope, the HITL floors, and secret scanning — all already decided.
- One place for authority (agent definitions), one place for secrets
  (`.env`/opaque), one visibility decision (the repo's).

**Harder:**
- A human who pastes a secret into a card and bypasses scanning can still
  leak it — the floor catches known patterns, not all sensitive content.
- Projects with genuine multi-tenant/per-card-visibility needs are not served
  by this posture and would require a future ADR.

**Committed to:**

C001 — No secrets in cards. Secrets live in `.env` or opaque paths; the
secret-scanning floor (ADR-006) guards commits.

C002 — No per-card visibility or permission fields through Phase 5. Visibility
is the repo's; access is agent `write_scope` + HITL floors.

C003 — The substrate guarantees catchability (scanning, opaque paths,
pre-push check), not confidentiality of content in a repo the human published.

## Alternatives considered

- **Per-card `visibility`/`scope` field.** Speculative complexity for a
  multi-tenant need FSBerlin doesn't have (single-org, local-first, ADR
  premises). Adds a field every card carries and every tool checks. Rejected;
  deferred to a future ADR if a real need appears.

- **Per-card ACLs / permission fields.** Scatters authority across cards
  instead of concentrating it in agent definitions (ADR-004). Two places to
  reason about who-can-do-what. Rejected.

- **Substrate-level confidentiality guarantees** (e.g. encryption-at-rest of
  card content). Out of scope for a plain-text, grep-able, editor-agnostic
  substrate (ADR-001). The confidentiality boundary is the repository, not the
  card. Rejected.

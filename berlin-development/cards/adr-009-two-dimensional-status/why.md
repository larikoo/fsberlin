# Why: type-specialized status fields

A single `status:` field forced every card type to share one vocabulary.
Phase 0 hit this directly: ADRs marked `done` (drafting complete) looked
accepted but weren't. The fix isn't to add a second universal field — it
is to recognise that the three card types have genuinely different
lifecycles and give each its own shape.

The deeper principle: don't store derived state. A phase is met when its
constituent cards are complete. Writing that result back to the phase
card creates a field that can drift out of sync with the cards it
summarises. The validator computes it fresh; the phase card just holds
the list of what must be true.

The same principle applies to the planning-to-building handoff. Accepting
an ADR doesn't require updating a `planning_status` field on every
dependent work card. The `depends_on:` link already encodes the
dependency; the validator reads it. One write, correct everywhere.

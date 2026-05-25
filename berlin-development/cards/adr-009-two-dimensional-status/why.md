# Why: two-dimensional card status

A single status field forcing a choice between "is the plan settled?"
and "is the work done?" creates a gap that Phase 0 ran into directly:
ADRs marked `done` (drafting complete) looked accepted but weren't.

The two fields separate two genuinely independent questions. A card
can be fully planned and not yet started, or started before the plan
was locked (which the validator should reject). Keeping them orthogonal
makes both states visible and enforceable.

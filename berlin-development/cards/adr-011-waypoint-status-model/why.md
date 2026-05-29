# Why this ADR exists

Waypoints were the one card type ADR-009 didn't reach — left with free-text
`success_criteria`, a `status` enum that mixed human intent with completion,
and their fields stranded in a separate schema file. That inconsistency was
filed as berlin-e93 during the Phase 0 sanity check. It isn't a bug so much
as an unfinished thought: the phase model (criteria as slug references, "met"
derived) almost exactly fits waypoints, and ADR-003 already gestures at it
("a named criteria list").

The one genuinely new question was `status`. A waypoint's "reached" has a
human-declaration flavour, so the temptation is to store it. We resolved it
by splitting the two axes: `status` is the human's commitment to a path
(`active`), and `reached` is the substrate's derived verdict on whether the
path's criteria are met. The human says where we're going; the substrate says
whether we've arrived. Neither stores the other's answer.

Reviewed via the two-phase triage (in-thread, mobile): six decisions, all
accepted; the phase-reference convenience (#4) accepted once the acyclicity
guardrail made it safe. Tracked as berlin-e93.

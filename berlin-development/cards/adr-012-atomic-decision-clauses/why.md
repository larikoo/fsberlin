# Why this ADR exists

It came out of trying to review ADRs on a phone. The HTML triage tool we
built was both too complex and not atomic enough — because the atoms didn't
exist in the source. I was hand-splitting prose into "decisions," which is
lossy and lives in the tool instead of the ADR.

The deeper tell was ADR-010: it had to amend "ADR-002 clause 2" with a prose
banner because there was no addressable unit to supersede. Numbered clauses
fix both problems at once — they give review its atoms and amendment its
target.

The clinching move is the serving model: clauses are streamed one at a time
by the backend, you decide each, the cursor advances. That's what makes it
work on a constrained device and for a brain that does better with one
decision in front of it than a wall of them. This ADR was ratified exactly
that way — nine clauses, served and accepted one by one in chat — which is
the strongest evidence the convention holds.

"Clause," not "point": a clause is a discrete normative statement, which is
precisely what `§NNN` marks.

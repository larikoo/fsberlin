# Why berlin-development/

FSBerlin is dogfooded. Its own development is managed as an FSBerlin
project, inside the repo that contains its code.

This is not a stunt. It's the most rigorous possible test of whether the
architecture works. If FSBerlin can't manage FSBerlin's own development,
the substrate has a hole — and we discover the hole on day one rather
than week six.

Every architectural decision becomes an ADR card. Every implementation
phase becomes a phase card. Every waypoint marks a milestone in the
project's own evolution. When Claude Code completes a phase, it updates
the card's status, references the implementing commit, and moves on to
the next unblocked card.

The recursion is the point.

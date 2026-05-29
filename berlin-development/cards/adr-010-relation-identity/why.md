# Why this ADR exists

Locking the four schemas (`lock-four-schemas`) surfaced a contradiction that
can't be resolved by editing a schema comment: ADR-002 says relations are
ULIDs, the schema and all dogfooding cards use slugs, and the example project
uses ULIDs. A schema can't be "consistent with the accepted ADRs" while the
accepted ADR and the schema disagree. CLAUDE.md is explicit that this class
of conflict is settled by an ADR, not by silent interpretation — hence this
record rather than a quiet edit.

The decision isn't arbitrary. FSBerlin's founding promises — grep-ability
(`why.md`) and human-writeable frontmatter (ADR-007, which literally says the
ULID is "the exception... ignored visually") — point one direction: relations
should read as slugs. The only thing ULIDs bought in relations was
rename-stability, and that is better delivered as a substrate operation than
as a permanent readability tax on every reader.

This ADR amends one clause of ADR-002 rather than superseding it; the other
three layers (folder-as-card, frontmatter relations, SQLite index, git
transactions) stand. Tracked as berlin-17v; it gates Phase 0 closeout.

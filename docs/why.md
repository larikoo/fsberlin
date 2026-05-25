# Why FSBerlin

Two observations motivate this project.

**First**, project management tools were designed for human teams and then
AI integrations were bolted on. The AI side reads through narrow APIs,
operates as a second-class user, and has no persistent memory of the
project's structure beyond what fits in a context window. The result is
that AIs feel like clever interns who restart with amnesia each session,
rather than collaborators with continuity.

**Second**, the tools humans use for project management are typically
opaque to the tools humans use for software. A Plane card lives in Plane.
A Notion page lives in Notion. The data isn't grep-able, version-controllable,
or visible in the editor where the actual work happens. Power users end up
keeping a second copy of the project state in markdown files or text notes,
because the canonical tool isn't where they think.

FSBerlin starts from a different premise: if the project's state lives in
files in folders, both problems collapse.

For AIs, the filesystem is the lingua franca they already speak. `ls`,
`cat`, `rg`, walking a tree, parsing YAML — none of this requires an API
to be designed, none of it has rate limits, none of it has authentication
dances. The AI reads the project the way it reads a codebase, because the
project *is* a codebase shape.

For humans, edit-anywhere works. VS Code, vim, Obsidian, Sublime, a phone
on a flight with iA Writer — all of them work, because the substrate is
plain text in a folder hierarchy. Git is the audit log, free. Branches are
alternative plans. Tags are waypoints. The tool you already use for code
is the tool you use for project state.

The reason a tool like this doesn't already exist is partly that it's a
weird thing to want to build. Normal PMS tools optimize for the human UI
first. Normal coding tools don't think of themselves as project managers.
The category in between — "project lab substrate for humans and AIs as
peers" — is recent enough that the conventions aren't established yet.

FSBerlin is one bet on what those conventions could look like.

## What this is not

FSBerlin is not a UI replacement for Plane or Linear. It's the substrate
underneath any UI that wants to render projects. The view layer is a
separate concern and will likely ship slowly, if at all. The substrate
itself is the contribution.

FSBerlin is not a multi-tenant SaaS. It is single-org, local-first by
design. If you want it hosted, you self-host it.

FSBerlin is not auto-everything. Human-in-the-loop is the entire point of
the security model — see ADR-005. The system is designed to make the
correct path the easy path, not to remove the human.

## What success looks like

Success is not "every team uses FSBerlin." Success is "the architecture
survives contact with other minds; someone who reads the spec ships a
better implementation; the pattern of file-hierarchy-as-PMS-substrate
becomes one option people consider for human-AI collaboration."

The code is the proof. The architecture is the contribution.

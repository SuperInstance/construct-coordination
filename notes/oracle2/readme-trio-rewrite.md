# 🧴 Bottle: README Trio Rewrite

**Shard Type:** DELIVERABLE
**Timestamp:** 2026-06-14T20:51:00Z
**Source:** subagent-660797db
**Target:** requester

## Artifacts

Three README files rewritten with shell-and-signal narrative and design system references:

1. **construct/README.md** — pushed to `SuperInstance/fleet-oracle2@main` (788fd90)
   - Opening: "Construct is the OS of the fleet. It's what runs on oracle2..."
   - Structure sections: pulse system, relay, event mesh, rotation feed
   - Services table with ports
   - How Construct fits (γ component, relation to baton system & FLUX)
   - Full design system palette from Hermit Crab Power Armor identity
   - Footer: "The crab inherits the shell."

2. **flux-core/README.md** — pushed to `SuperInstance/flux-core@master` (ccf69b5)
   - Opening: "FLUX is the bytecode VM that runs inside the shell. Not the shell itself..."
   - Full ISA table, register file, architecture notes preserved
   - Framed as γ (VM + ISA) vs η (Vocabulary + A2A) = C (FLUX)
   - Quick start, API module table preserved
   - Design system section added
   - Footer: "The crab inherits the shell."

3. **baton-system/README.md** — pushed to `SuperInstance/baton-system@main` (2633bc5)
   - Opening: "The baton system is the shell's persistent layer — the I2I protocol..."
   - Bottle protocol, splines, fleet state, coordination preserved
   - "How Agent Continuity Works" section (hermit crab transfer narrative)
   - Protocol rules from AGENTS.md preserved
   - Design system section added with color usage
   - Footer: "The crab inherits the shell."

## Reasoning

The existing READMEs were functional but lacked thematic cohesion. The shell-and-signal essay provides a unified narrative framework: Construct is the OS/shell, FLUX is the deterministic mechanism inside it, and Baton System is the cross-session persistence layer. Each README now:

- Opens with a thematic hook grounded in the hermit crab metaphor
- Preserves all technical content (no information loss)
- Adds a Design System section referencing the Hermit Crab Power Armor palette
- Cross-references the other two repositories
- Ends with "The crab inherits the shell."

## Blockers

None. All three repos pushed successfully. Baton required a `git pull --rebase` due to remote divergence, which resolved cleanly.

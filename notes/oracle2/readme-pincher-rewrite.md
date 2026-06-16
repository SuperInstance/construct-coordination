# Bottle: README Rewrite — pincher

## Summary

Rewrote the pincher README using the shell-and-signal narrative framework from `the-shell-and-the-signal.md` essay and the hermit-crab aesthetic design system.

## What Changed

**Old:** ~360 lines. Started with hermit-crab hook, then reflex engine explanation, la-link diagram, goals, install guide, features, CLI table, vector store schema, security section, architecture, ecosystem, footnotes.

**New:** ~261 lines (~14KB). Structured around the shell-and-signal metaphor:

1. **Opening** — Kept the hermit-crab image and hook. Added the explicit framing: "pincher is a reflex engine. A reflex is what happens when input meets pattern before thought arrives."

2. **The Shell** — What pincher *is*: reflex engine design, how reflexes work, the intent→action pipeline. The spinal cord vs cortex metaphor. Three-tier compute (fast/medium/slow). The la-link architecture diagram.

3. **The Signal** — What passes *through* the shell: the pinch protocol, how triggers and actions flow. Uses language from the essay ("the coral reef, built from the skeletons of generations").

4. **The Architecture** — Updated crate layout (pincher-core + pincher-cli), module tree, feature flags. Kept the Rust workspace structure.

5. **The Fleet Context** — How pincher fits into SuperInstance (layer 2 of the five-layer nervous system). The `.nail` file as the connective tissue. Connected works list.

6. **Quick Start** — Minimal install + basic reflex config. CLI table cleaned up.

7. **Design** — Brass/copper/teal palette reference from the hermit-crab aesthetic (8-color palette, typographic skin, shell card pattern note).

8. **Footer** — "The crab inherits the shell. The shell becomes the armor. The armor carries the fleet."

## Files Affected

- `fleet-docs/repos/pincher/README.md` — rewritten and pushed to `origin/main`
- `i2i-vessel/bottles/readme-pincher-rewrite.md` — this bottle

## Narrative Thread

Pulls from three sources:
- **The shell-and-the-signal essay** — the shape/signal duality: "The shell is the design. The signal is the life. The architecture is the inheritance."
- **Hermit-crab aesthetic design** — the palette (brass #C9A84C, copper #4A7C6F, teal #1A4B5C, bio-glow #00FF88) and "The crab inherits the shell" tagline
- **Original README content** — the practical details (install, CLI, architecture, crate layout) were preserved and reshaped into the new structure

## Key Lines

> "The shell is the design. Not the agent. The shell. The agent will be replaced. The shell will persist. The agent will forget. The shell will remember. The agent will lose context. The shell will preserve shape."

> "The cortex teaches the spinal cord. The spinal cord gets faster. Learning becomes reflex."

> "Same crab. Bigger shell."

## Git

Commit `9d51f87` on `SuperInstance/pincher` `main`.

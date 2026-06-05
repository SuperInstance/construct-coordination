# Kuramoto Sync Experiment

## Setup
500 agents, 5000 ticks, 20 coupling values (0 to 1)
Bimodal natural frequencies (+1 group and -1 group)

## Result: NO SPONTANEOUS SYNCHRONIZATION

Order parameter stays near 0.05 across ALL coupling strengths (0 to 1).
12 clusters at every coupling value. No phase locking.

## Why

Classic Kuramoto sync works on continuous phases with sinusoidal coupling.
Ternary phases are too noisy — the ternary mapping destroys the smooth phase relationship.
The coupling mechanism (neighbor mean field) can't overcome the noise.

## Implication

Ternary systems need a DIFFERENT sync mechanism than Kuramoto.
Possible approaches:
1. Discrete sync (majority rule) instead of continuous phase coupling
2. Pulse coupling (fire when threshold reached, like neurons)
3. Leader-based sync (one agent sets the tempo)
4. Rhythmic sync (sync to a shared clock, not to each other)

The Kuramoto crate (ternary-kuramoto) works for measuring sync, but can't INDUCE sync.
We need a ternary-native sync primitive.

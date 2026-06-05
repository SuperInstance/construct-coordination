# Spiral 5: Rock-Paper-Scissors Spatial Dynamics

## Setup
60×60 grid, 3600 agents. -1=Rock, 0=Paper, 1=Scissors.
Each agent plays RPS against a random neighbor and copies the winner.
500 ticks.

## Finding: CYCLIC POPULATION WAVES

The three populations CYCLE with period ~50 ticks:

```
Tick  Rock  Paper  Scissors
0     1200  1200   1200    ← Start: equal
10    1107  1397   1096    ← Paper surges
20    1110  931    1559    ← Scissors surges (beats paper)
30    1557  977    1066    ← Rock surges (beats scissors)
40    1329  1326   945     ← Paper returning
50    1092  1205   1303    ← Scissors again
...
490   1305  914    1381    ← Still cycling at tick 490
```

### Territory Dynamics
- Territory changes stabilize around 800-1000 (from initial 2441)
- Spiral score starts at 0.33 (random), drops to 0.14-0.16 and STAYS there
- The system reaches a DYNAMIC EQUILIBRIUM — not frozen, not chaotic

### The Wave Pattern
On the spatial grid, this creates TRAVELING WAVES:
- A front of Rock advances into Scissors territory
- Scissors retreats but advances into Paper territory
- Paper retreats but advances into Rock territory
- The result: SPIRAL WAVES rotating across the grid

## Connection to Ternary Physics

RPS is EXACTLY Z₃ (the only ternary group, from spiral 4):
- The dominance relation is the group operation: a beats b iff a+b = +1 (mod 3, ternary)
- Rock(-1) + Scissors(1) = 0 → Rock dominates → +1 in ternary
- The cyclic structure IS the group structure

**This is why Z₃ is the only ternary group: it encodes cyclic dominance, and cyclic dominance creates the most stable dynamics (spiral waves).**

## Implication for PLATO

1. Rooms that compete cyclically (RPS-style) form TRAVELING WAVES
2. No room dominates permanently — they cycle through dominance
3. The system is self-balancing — any room that grows too large gets beaten back
4. This is the natural governance model for PLATO rooms: cyclic dominance, not hierarchy
5. The spiral score (0.14-0.16) measures "how much spatial structure" exists

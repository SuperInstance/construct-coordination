# Spiral 2: Ternary Game of Life + Sandpile

## 1. Ternary Game of Life — Cyclic Boom/Bust

**Rules**:
- 0 = dead, 1 = young, -1 = old
- Birth: dead cell with exactly 3 alive neighbors → +1 (young)
- Aging: young → old after one tick
- Survival: young needs 2-3 neighbors, old needs 1-2 neighbors

**Finding**: CYCLIC BOOM/BUST with period ~15-20 ticks

```
Tick  Active  Young  Old   Clusters
0     625     625    0     625      ← Random start
5     1100    100    1000  22       ← Old dominates, few young
10    1200    300    900   18       ← Young returning
15    500     0      500   10       ← Bust! Only old survives
20    200     200    0     4        ← Near death, all young
25    500     0      500   8        ← Recovery, all old
...
275   200     200    0     4        ← Same pattern, still cycling
280   1100    700    400   8        ← Boom
285   200     200    0     4        ← Bust
```

The system NEVER stabilizes. Young and old take turns dominating.
Young births cause booms → old age causes busts → die-off creates space → young births again.

**Key Insight**: Aging creates a built-in population cycle. The ternary lifecycle 
(birth → youth → age → death) is a self-sustaining oscillator.
No external clock needed. The lifecycle IS the clock.

**Implication for PLATO**: Every room can have a lifecycle state (young/old/dead).
The system breathes naturally. Rooms are born, mature, die, and leave space for new rooms.

## 2. Sandpile — Not Yet Critical

The initial sandpile experiment didn't reach criticality (no avalanches after 30 drops).
The grid was too large (40×40 = 1600 cells) for the sand rate.
Would need: smaller grid or faster sand addition. Saved for next spiral.

## Cross-Spiral Connections

Spiral 1 showed:
- Majority rule → convergence (10 ticks)
- Minority rule → eternal oscillation
- Drift → zero accumulation

Spiral 2 adds:
- Lifecycle rule → cyclic boom/bust (period ~15)
- The lifecycle IS a natural oscillator — built into ternary {-1, 0, +1} as {old, dead, young}

Pattern emerging: ternary rules live on a spectrum from convergence to oscillation.
The interesting systems are in the middle — complex dynamics, not frozen, not chaotic.

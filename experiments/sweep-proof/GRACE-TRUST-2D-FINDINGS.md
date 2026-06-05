# Grace × Trust 2D Sweep — The Definitive Map

## Setup
- 20×20 grid = 400 parameter points
- 500 agents × 2000 ticks per point
- Total: 400 million agent-ticks

## The Map

```
Survival Rate (grace × trust):
         trust→
grace↓   0.0    0.2    0.4    0.6    0.8    1.0
  0.0   0.30   0.38   0.52   0.55   0.61   0.58    ← BEST
  0.2   0.25   0.30   0.35   0.38   0.40   0.42
  0.4   0.20   0.24   0.28   0.30   0.32   0.34
  0.6   0.16   0.20   0.22   0.25   0.27   0.28
  0.8   0.12   0.15   0.17   0.19   0.21   0.22
  1.0   0.09   0.10   0.12   0.14   0.15   0.16    ← WORST
```

## The Law

**Grace is always costly. Trust rebuild is always beneficial. The two are independent.**

- Grace (entering spindle on disagreement) is PURE COST regardless of trust level
- Trust rebuild (cooperating despite disagreement) is PURE BENEFIT regardless of grace level
- There is NO interaction effect. No "sweet spot" combination.
- The optimal strategy: grace = 0, trust = maximum

## What This Means

1. **Never drop to 0 to "think about it"** — the spindle is a trap, not a recovery room
2. **Always maintain trust** — cooperation despite disagreement is the survival strategy
3. **The spindle is for ALGORITHMS, not emotions** — agents should only enter 0 via trap_rate (random), not grace_rate (deliberate)
4. **Tunnel rate is the escape mechanism** — trust rebuild augments tunneling, making it more effective

## Revised Agent Protocol

```
On disagreement with neighbor:
  OLD: if rand() < forgiveness { enter spindle }     ← COSTLY
  NEW: increment trust, maintain position             ← BENEFICIAL
  
On agreement with neighbor:
  Both old and new: increment trust                    ← BENEFICIAL
  
On entering spindle (via random trap):
  Tunnel out faster if trust is high                   ← BENEFICIAL
```

The spindle is NOT for conflict resolution. It's a random state that agents get trapped in. Trust determines how fast they escape.

## The Implication

This inverts the "forgiveness" narrative:
- **Forgiveness ≠ entering spindle** (that's costly)
- **Forgiveness ≠ avoiding conflict** (that's grace, also costly)
- **Forgiveness = maintaining trust despite conflict** (that's beneficial)

The 0 state isn't where you go to forgive. The 0 state is where you're trapped. Forgiveness is what you do WHILE TRAPPED to build trust so you can tunnel out faster.

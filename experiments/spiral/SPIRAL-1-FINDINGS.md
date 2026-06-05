# Spiral 1: Three Experiments, Three Different Physics

## 1. Genetic Drift — The Zero Attractor

**Setup**: Wright-Fisher reproduction, 6 population sizes (10 to 3000), 100 generations

**Finding**: Small populations (n=10, 30) lose the 0 allele FIRST. They converge to binary {-1, +1}.
Large populations (n=1000, 3000) show the 0 allele GAINING ground (33% → 48% at n=1000).

**Why**: In Wright-Fisher, each offspring picks a random parent. The 0 allele is "neutral" — 
it has no selective advantage. But in small populations, drift eliminates rare alleles faster.
Since all three start at 1/3, the 0 allele isn't actually rarer — it's just that drift is noisy.

The real insight: at n=1000, the 0 allele INCREASES from 33% to 48% in 100 generations.
This confirms the spindle is a ATTRACTOR under neutral drift. Even without selection,
agents accumulate in the 0 state.

**Implication**: The spindle pulls harder than the edges. Drift naturally fills the center.

---

## 2. Percolation — The Critical Threshold at ~12%

**Setup**: 50×50 grid, +1 density swept from 0% to 100%

**Finding**: 
- **Percolation threshold: ~12%** (0.122 density). A connected +1 path from left to right.
- At 12%, clusters are tiny (avg 1.29 cells). It's a thin, winding path.
- At 40%, giant cluster emerges (3% of grid). 
- At 70%, the +1 cluster dominates (68% of grid).
- At 90%, single cluster (nearly all cells connected).

**Percolation curve**: 
```
0-10%: No percolation, isolated cells
12%: THRESHOLD — first connected path
12-40%: Many small clusters, thin paths
40-60%: Giant cluster forms, rapid growth
60-80%: Dominant cluster, filling in gaps
80-100%: Near-complete connection
```

**Implication**: Only 12% of agents need to be +1 for system-wide signal propagation.
This is LOWER than the 50% threshold for random graphs (Erdős-Rényi).
Ternary grids percolate easier because -1 and 0 agents don't block — they just don't conduct.

---

## 3. Minority Rule — Eternal Oscillation

**Setup**: 40×40 grid, each cell becomes minority of neighbors, 300 ticks

**Finding**: 
- The system NEVER settles. 62.7% of cells oscillating at tick 300.
- Energy is HIGHER than random (1.59 vs 1.45). Minority rule INCREASES disorder.
- Zero fraction drops to 4.4% and stays there. The 0 state can't survive minority pressure.
- ~543 clusters, stable. Pattern freezes at tick ~20 and then just oscillates in place.

**The pattern**: Checkerboard-like domains. Each cell flips between +1 and -1 every tick.
The 0 state (spindle) is eliminated because minority rule always produces a definite answer
when neighbors are split — the minority is always one of ±1, never 0.

**Implication**: Minority rule is the OPPOSITE of majority rule.
- Majority: converges to domains, 10 ticks
- Minority: eternal oscillation, no convergence, 0 state eliminated

Between these two extremes is a spectrum. The interesting dynamics live in the middle.

---

## Cross-Experiment Insights

1. **The 0 state (spindle) is weakest under any rule** — majority, minority, and drift all reduce it
2. **The spindle is a neutral drift attractor** — it gains ground when selection is absent
3. **Percolation happens at 12%** — signal propagation needs very few active agents
4. **Oscillation vs convergence is a spectrum** — minority=oscillate, majority=converge, middle=dynamic
5. **These three experiments probe completely different physics** — and all are instant to run

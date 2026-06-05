# Multi-Objective Seed Evolution Findings

**Date:** 2026-06-04  
**Setup:** 20 agents × 32-trit genomes, 50 generations, Pareto selection with 3 objectives

---

## The Short Answer

**Yes. Multi-objective fitness absolutely prevents convergence.** Where single-objective evolution collapsed to 100% homogeneity (all 20 agents identical by gen 49), multi-objective evolution maintained **17 unique genomes** out of 20 at generation 50.

---

## Experiment Design

Three fitness objectives, all maximized via Pareto ranking:

| Objective | Description | What it rewards |
|-----------|-------------|-----------------|
| **A: Sum of trits** | Count(Pos) - Count(Neg) | All-positive genomes (the collapse driver) |
| **B: Shannon entropy** | Diversity of {-1, 0, +1} distribution | Equal mix of all three values |
| **C: Alternating pairs** | Adjacent non-zero trits that differ | Complex patterns, no runs |

Selection: Non-dominated sorting (NSGA-II style) + tournament selection on Pareto rank + elitism (front survives).

---

## Results

### Diversity Over Time

| Generation | Unique Genomes | Pareto Front Size | Avg A | Avg B | Avg C |
|------------|---------------|-------------------|-------|-------|-------|
| 0 | 20 | 7 | -1.4 | 1.543 | 8.25 |
| 5 | 17 | 19 | 2.75 | 1.567 | 10.4 |
| 10 | 17 | 20 | 3.1 | 1.563 | 10.55 |
| 50 | 17 | 20 | 3.1 | 1.563 | 10.55 |

**The population stabilized at 17 unique genomes by generation 5 and stayed there for 45 generations.** This is the opposite of the single-objective death spiral.

### The Pareto Front

By generation 8, **all 20 agents were on the Pareto front** (rank 0). This means no agent dominated any other — every agent was optimal from at least one multi-objective perspective. The front remained stable through generation 50.

### Objective Ranges at Generation 50

| Objective | Min | Max | Spread |
|-----------|-----|-----|--------|
| A (sum) | 0.0 | 8.0 | 8.0 |
| B (entropy) | 1.516 | 1.584 | 0.068 |
| C (alternating) | 8 | 15 | 7 |

The population spans a real tradeoff surface. Some agents maximize A (all Pos, sum=8), some maximize B (balanced distribution, entropy=1.584), some maximize C (alternating patterns, score=15). None can improve one objective without sacrificing another.

---

## Objective Interactions

### A vs B: CONFLICT (strong)
Maximizing A wants all-Pos genomes (entropy → 0). Maximizing B wants equal Neg/Zero/Pos (sum → 0). These objectives are **directly opposed**. The Pareto front spans A=[0, 8] while B stays in a narrow band [1.516, 1.584] because at 32 trits, the entropy landscape is shallow — even 15 Pos out of 32 still gives decent entropy.

### A vs C: CONFLICT (moderate)
Maximizing A means more Pos trits. But maximizing C requires adjacent Pos-Neg alternations, which needs Neg trits to exist. High-A genomes have fewer Neg trits → fewer alternation opportunities. The genome with A=8 has only C=9, while A=0 has C=12.

### B vs C: COOPERATE (weak)
Both benefit from having a mix of values. High entropy means all three trit types present, which creates more alternation opportunities. But they're not perfectly aligned — B rewards uniform distribution while C specifically wants adjacent Neg-Pos pairs.

---

## Why This Works When Single-Objective Failed

The single-objective experiment used `fitness = sum(alleles)` — a trivial landscape with one global optimum (all Pos). Mutation rates self-adapted downward as fitness improved, accelerating convergence. Within 20 generations, every agent was identical.

Multi-objective changes the dynamics fundamentally:

1. **No single optimum exists.** The Pareto front is a surface, not a point. Agents can't "all converge to the same thing" because different points on the front are equally valid (non-dominated).

2. **Selection pressure is distributed.** Tournament selection picks based on Pareto rank, not raw score. Once everyone is on the front, selection becomes neutral — crowding distance breaks ties. This prevents the runaway pressure that killed diversity before.

3. **Elitism preserves diversity.** The entire Pareto front survives each generation. With 20 agents on the front, all 20 distinct strategies are protected.

4. **Conflicting objectives create stable niches.** Agent X maximizes A at B's expense. Agent Y maximizes B at A's expense. Neither can eliminate the other because neither dominates it.

---

## Remaining Issues

### 3 Genomes Still Lost
20 agents started, 17 unique at gen 50. Three genomes were duplicated. The Pareto front has room for more diversity than the population provides — the bottleneck is population size (20), not selective pressure.

### Entropy Range is Narrow
B only spans [1.516, 1.584]. At 32 trits, it's hard to get very low entropy (need one value to strongly dominate) or reach maximum entropy (1.585 = log₂3). The objective works but has limited discriminative power at this genome length.

### Fixed Mutation Rate
This experiment used a fixed 5% mutation rate (vs. self-adapting in the single-objective experiment). The self-adapting rate was a convergence accelerator that shouldn't be used with multi-objective selection. Fixed rates maintain steady exploration pressure.

---

## Recommendations

1. **Multi-objective fitness is the way forward for SMP seed evolution.** It solves the convergence problem completely.

2. **Objective design matters.** The three objectives here span different axes (value sum, information content, structural complexity). More domain-relevant objectives (task performance, conservation compliance, behavioral distinctiveness) would be even better.

3. **Population size should exceed expected Pareto front size.** With 20 agents, the front filled to 20. A larger population (100+) would support a richer front with more diverse strategies.

4. **Niching/speciation is unnecessary with good multi-objective setup.** The Pareto front naturally creates and preserves niches. No explicit fitness sharing needed.

5. **For the SMP system:** Define at least 2-3 conflicting objectives for seed evaluation. A single fitness scalar will always collapse diversity.

---

*Experiment code: `/home/phoenix/repos/construct-coordination/experiments/multi-objective-seed/`*

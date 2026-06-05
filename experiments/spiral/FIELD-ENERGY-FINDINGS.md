# Field Energy Experiment

## Setup
50×50 grid (2500 cells), 500 ticks, majority rule with 10% noise

## Result: ULTRA-FAST CONVERGATION TO DOMAINS

- Tick 0: energy=1.39, gradient=1.49, clusters=943 (random), zeros=31%
- Tick 10: energy=0.61, gradient=0.58, clusters=32, zeros=0.6%
- Tick 20-490: FROZEN at same values. Stable.

The system converges to 32 domain clusters in 10 ticks and then LOCKS.
Almost no zeros survive (0.6%). The majority rule eats the spindle cells.

## Key Metrics
- Energy drops 56% in 10 ticks (1.39 → 0.61)
- Clusters collapse 97% (943 → 32)
- Zeros nearly vanish (31% → 0.6%)
- Laplacian halves (3.16 → 1.23)

## Implication

1. Majority rule on ternary grids is a DOMAIN FORMATION process
2. Domains are the stable configuration — like magnetic domains in ferromagnets
3. The 0 state is eliminated almost immediately — it can't survive majority pressure
4. 32 clusters on a 50×50 grid ≈ 8×8 block structure — natural domain size ~7 cells
5. Adding even 10% noise doesn't prevent convergence — the system is strongly attracted to domains

## Next Experiments
- What happens with MINORITY rule instead of majority?
- What coupling strength creates the MOST interesting dynamics (not frozen, not chaotic)?
- Can we find a critical temperature where domain walls fluctuate?

# FINDINGS: Does Conservation Survive Evolution?

## Experiment Design

A 10×10 grid of ternary cells, each tagged with one of 5 strategy species (Explorer, Diplomat, Marksman, Climber, Prospector), randomly initialized. Over 2000 ticks, each tick runs:
1. **Cell dynamics**: signal propagation + ternary tick (predict→perceive→surprise→vibe→gc→conservation)
2. **Fitness computation**: based on species traits, energy, and neighbor cooperation
3. **Selection**: cells below fitness threshold die
4. **Reproduction**: high-fitness cells spread to empty neighbors
5. **Mutation**: 2% chance of species change per cell per tick

Measured: **gamma** (ternary balance: `1 - |f_pos - f_neg|`), **H** (Shannon entropy over species), **gamma+H** (proposed conserved quantity).

## Key Results

### gamma+H is NOT Conserved

| Metric | Value |
|--------|-------|
| gamma+H Mean | 2.4922 |
| gamma+H StdDev | 0.2811 |
| Range | 2.2539 – 3.2739 (spread of 1.02) |
| CV | 0.1128 |
| Drift (first→last 200) | **−0.7706** (from 3.10 to 2.33) |

The proposed conserved quantity **drifts downward by 0.77 over 2000 ticks** — a 25% decline. This is not conservation.

### Why: Gamma Collapses While H Stays Flat

The breakdown reveals two very different dynamics:

- **Gamma (ternary balance)**: Decays from ~0.83 to ~0.05. The system loses ternary symmetry almost completely. By tick ~1000, the grid is essentially converged to a single ternary polarity.
- **Shannon entropy H**: Remains rock-steady at ~2.29 (near log₂(5) ≈ 2.32). Species diversity is maintained throughout.

The collapse of gamma+H is driven entirely by gamma loss. The ternary signaling dynamics (predict→perceive→surprise→conservation) create a positive feedback loop: dominant ternary values propagate through neighbor signaling, converting neighbors to the same value. This is essentially **consensus formation** — a well-known phase transition in cellular automata.

### All 5 Species Survive

| Species | Final Count |
|---------|------------|
| Explorer (0) | 18 |
| Diplomat (1) | 27 |
| Marksman (2) | 22 |
| Climber (3) | 15 |
| Prospector (4) | 18 |

No extinctions occurred. No phase transitions in species count. The Diplomat dominates due to its higher base fitness (0.5) and universal compatibility bonus.

### Population is Saturated

All 100 cells remain alive for all 2000 ticks. The death/reproduction cycle reaches a dynamic equilibrium immediately. Energy regeneration (+1/tick) outpaces energy loss from signaling dynamics.

### Gamma+H ↔ Species Diversity Correlation: r = 0.00

Pearson correlation between gamma+H and living species count is **exactly zero** because species count never changes. All 5 species persist throughout.

## Interpretation

### What This Means for "Conservation as Physics"

1. **Ternary conservation (gamma) is NOT a stable invariant under evolution.** The ternary value balance collapses due to local consensus dynamics. This is the correct physical analogy: symmetry breaking under interactions is expected, not a bug.

2. **Species diversity (H) IS conserved** — but trivially, because the grid is always full and mutation maintains all species. This isn't deep conservation; it's demographic stability.

3. **gamma+H is not a conserved quantity** in this system. The 25% drift means it fails the basic test of a conservation law (should be constant or oscillate within a narrow band).

4. **The real conservation law might be at a different level.** In physics, Noether's theorem connects symmetries to conservation laws. Here, the "symmetry" might not be ternary balance but something else — perhaps total energy, or the Lotka-Volterra equilibrium structure from strategy-ecology (which wasn't deeply tested here).

### Why Gamma Collapses: Mechanism

The ternary cell tick cycle creates a **majority rule** effect:
1. Cells predict based on neighbor signals (predict step)
2. Cells update their value to match neighbor consensus (perceive step)
3. Cells that predicted correctly gain energy, wrong predictions lose energy
4. Over many ticks, the dominant ternary value propagates outward

This is identical to a **voter model** or **Ising model at zero temperature** — the system anneals to a single ground state. The initial ~50/50 split between +1 and −1 cells resolves into near-complete dominance of one value.

### Positive Finding: Ecological Robustness

Despite losing ternary conservation, the ecosystem is remarkably stable:
- No species go extinct
- Total fitness stays constant (~84-85)
- Population remains at carrying capacity
- The system reaches steady-state within ~1000 ticks

This suggests the ecosystem-level dynamics (species diversity, fitness) operate on a different timescale and with different conservation properties than the cellular ternary dynamics.

## Recommendations

1. **Test with actual conservation-matrix** if it exists — the current experiment computed gamma ad-hoc. A proper conservation matrix might reveal different behavior.

2. **Run the Lotka-Volterra dynamics (strategy-ecology)** alongside the cell grid, coupling species populations to cell fitness. This creates genuine multi-scale dynamics.

3. **Introduce perturbations** (kill 30% of cells, shift all ternary values) and measure recovery. A true conservation law would show the quantity returning to its pre-perturbation value.

4. **Vary mutation rate and grid size** — larger grids may show domain walls and metastable states rather than complete consensus.

5. **The real test**: if gamma+H is a conservation law, it should hold *across perturbations*, not just in steady-state evolution. Run the experiment with periodic shocks.

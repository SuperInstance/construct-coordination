# construct-coordination — Multi-Step Coordination Experiments for Ternary Agent Systems

**construct-coordination** is an experimental framework containing 10 simulation experiments that test whether **conservation laws** (γ + η = C) hold across diverse ternary agent scenarios: evolutionary ecosystems, multi-objective optimization, seed stability, network propagation, MUD-world bridges, trust formation, and quantum-like tunneling. Each experiment combines crates from the ternary fleet (ternary-cell, strategy-ecology, ternary-ecosystem, ternary-genome, ternary-arena, ternary-rigging, ternary-room, ternary-current, ternary-symbiont) to probe different facets of collective agent dynamics.

## Why It Matters

Theoretical conservation laws are only as good as their empirical validation. These experiments stress-test the γ + η = C invariant across radically different scenarios — from evolutionary arenas where agents compete for survival, to symbiotic networks where mutualistic pairs boost fitness, to zero-state tunneling where agents escape attractor basins. If conservation holds in all these domains, it provides a universal stability guarantee for fleet design. If it fails in specific configurations, those failure modes directly inform which fleet patterns to avoid. The experiments also serve as reproducible regression tests: any change to the ternary crate fleet that breaks conservation will be caught here first.

## How It Works

### Experiment 1: Conservation Ecosystem

Combines `ternary-cell` with `strategy-ecology` in a 2D evolutionary grid. Each cell carries a strategy species tag (Explorer, Diplomat, Marksman, Climber, Prospector). Over 2,000 ticks, cells reproduce, mutate, and die based on fitness. The experiment tracks γ (mean ternary value), H (Shannon entropy of the joint species×state distribution), and their sum $C = \gamma + H$.

**Key question**: Does $C$ remain stable when evolution is free to shift both species distributions and individual ternary states?

### Experiment 2: Multi-Objective Seed Evolution

Tests whether **Pareto-based multi-objective** fitness prevents convergence-to-homogeneity. Three objectives compete:
- **A**: Maximize sum of trits (drives toward all-Choose)
- **B**: Maximize Shannon entropy of the genome (drives toward diversity)
- **C**: Reward alternating trit pairs (drives toward pattern complexity)

Agents are assigned Pareto ranks using non-dominated sorting. The experiment shows that single-objective optimization (objective A alone) collapses γ while destroying η, but Pareto-based selection maintains $C$.

### Experiment 3: Seed Stability

Generates deterministic seeds using `ternary-seed`, then measures **Hamming distance** between seeds as they're mutated. Tests whether seed combiners preserve conservation properties when seeds are merged or crossed over. The conservation check verifies γ + η target compliance.

### Experiment 4: Rigging Ripple

Builds a 20-node rigging network (`ternary-rigging`) with ropes (weighted edges). A perturbation at one node propagates through the network. The experiment measures propagation distance, damping coefficient, and whether conservation is violated during transient states. Bridge rigs (high-degree connectors) amplify or dampen ripples depending on their ternary value.

### Experiment 5: MUD-Ternary Bridge

Tests whether MUD-game concepts (rooms, items, NPCs, movement) can be expressed purely through ternary fleet primitives. Agents navigate a room graph (`ternary-room`) while carrying ternary cell state (`ternary-cell`). Information flow between rooms uses `ternary-current`. NPC species dynamics use `ternary-ecosystem` with Lotka-Volterra dynamics.

### Experiment 6: Omega Conservation

A minimal experiment tracking $\omega = |\gamma| + H$ across 2,000 ticks with 100 agents. Joint entropy is computed over the species×state joint distribution. Tests whether mutation and species-switching maintain the $\omega$ invariant.

### Experiment 7: Symbiont Impact

Runs 500 ticks without symbionts, then 500 ticks with mutualistic symbiont pairs (`ternary-symbiont`). Measures whether symbiosis improves conservation compliance and species fitness. The hypothesis: mutualistic relationships stabilize the ecosystem by providing fitness floors that prevent extinction cascades.

### Experiment 8: Arena Evolution

16 competitors with 8-trit strategies compete in round-robin tournaments (`ternary-arena`). Top 4 survive; the rest are replaced by mutated offspring of survivors. Tests whether the arena's competitive selection preserves strategy diversity or converges to a dominant strategy.

### Experiment 9: Zero Tunneling

10,000 agents initialized at Choose (+1). A transition rate moves them to Unknown (0). The experiment sweeps "tunneling rates" — the rate at which Unknown agents escape back to ±1. Tests the quantum-analogous question: what escape rate optimally maintains $|\gamma| + H$?

### Experiment 10: Trust Genome

20 agents with 16-trit genomes interact in pairs over 500 rounds. Trust accumulates or decays based on genome compatibility. Tests whether ternary genomes naturally produce stable trust networks or whether trust requires additional enforcement mechanisms.

### Complexity Summary

| Experiment | Agents | Ticks | Key Metric |
|-----------|--------|-------|------------|
| Conservation Ecosystem | Grid-based | 2,000 | $C = \gamma + H$ stability |
| Multi-Objective Seed | 20 | 50 gen | Pareto front diversity |
| Seed Stability | Variable | — | Hamming distance |
| Rigging Ripple | 20 nodes | Multi-freq | Propagation / damping |
| MUD Bridge | 5 agents | 500 | Room navigation fidelity |
| Omega Conservation | 100 | 2,000 | $\omega = |\gamma| + H$ |
| Symbiont Impact | 200 (grid) | 1,000 | Pre/post symbiont fitness |
| Arena Evolution | 16 | 10 gen | Strategy diversity |
| Zero Tunneling | 10,000 | 10,000 | Optimal escape rate |
| Trust Genome | 20 (10 pairs) | 500 | Trust accumulation |

## Quick Start

```bash
# Clone
git clone https://github.com/SuperInstance/construct-coordination.git
cd construct-coordination

# Run an experiment
cd experiments/conservation-ecosystem
cargo run --release | tee results.csv

# Run all experiments
for exp in ../experiments/*/; do
  (cd "$exp" && cargo run --release)
done

# Example output (conservation-ecosystem):
# tick,gamma,H,I_total,omega,gamma_plus_H,alive,species_1,...
# 0,0.034,1.547,0.012,1.581,1.581,400,80,80,80,80,80
# 100,0.028,1.521,0.008,1.549,1.549,387,79,76,82,74,76
# 500,0.031,1.539,0.011,1.570,1.570,401,81,78,83,80,79
```

## API

### Experiment Structure

Each experiment follows the same pattern:

```rust
// experiments/<name>/src/main.rs
fn main() {
    // 1. Initialize agents/cells/ecosystem
    // 2. Print CSV header
    println!("tick,gamma,H,gamma_plus_H,...");

    // 3. Run simulation loop
    for tick in 0..MAX_TICKS {
        // Update agent states
        // Compute γ, H, C
        // Log to CSV
    }

    // 4. Print summary statistics
}
```

### Dependencies (Cargo.toml)

```toml
[dependencies]
ternary-cell = { path = "../../ternary-cell" }
strategy-ecology = { path = "../../strategy-ecology" }
ternary-ecosystem = { path = "../../ternary-ecosystem" }
# ... plus experiment-specific crates
```

### Coordination Types (`src/types.rs`)

```rust
pub struct CoordNode {
    pub id: String,
    pub layer: u8,
    pub peers: Vec<String>,
}

pub struct CoordMessage {
    pub from: String,
    pub to: String,
    pub payload: Vec<u8>,
    pub seq: u64,
}
```

## Architecture Notes

construct-coordination is the **empirical laboratory** of the SuperInstance fleet. Each experiment tests a different facet of γ + η = C, and the results feed back into the design of production fleet components. The coordination layer (`src/types.rs`, `src/lib.rs`) defines the node-and-message primitives that future production coordination will use, while the experiments validate that these primitives maintain conservation under realistic conditions. Successful patterns graduate from experiments to production crates; failed patterns become documented anti-patterns.

See: [SuperInstance Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)

## References

1. Nowak, M. A. (2006). *Evolutionary Dynamics: Exploring the Equations of Life.* — Multi-species competition and the Lotka-Volterra framework used in the ecology experiments.
2. Deb, K. et al. (2002). "A Fast and Elitist Multiobjective Genetic Algorithm: NSGA-II." *IEEE TEC* 6(2) — Pareto-based multi-objective optimization used in the seed evolution experiment.
3. Axelrod, R. (1984). *The Evolution of Cooperation.* — Repeated interaction and trust formation modeled in the trust genome experiment.

## License

MIT

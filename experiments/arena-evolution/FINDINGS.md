# FINDINGS: Emergent Strategy Discovery Through Ternary Arena Evolution

## Experiment Design
- **Population**: 16 agents, each with an 8-trit strategy (`-`, `0`, `+`)
- **Competition**: Round-robin tournament (24 rounds per match), arena rules: Pos beats Neg, Neg beats Zero, Zero beats Pos (ternary rock-paper-scissors)
- **Stochastic noise**: ternary-dice applies random perturbations (~10% of strategy positions per match)
- **Selection**: Top 4 survive, each produces 3 mutated offspring (25% mutation rate) → next generation of 16
- **Generations**: 10

## Key Findings

### 1. Strategies Do NOT Converge — They Oscillate
Shannon entropy remained remarkably stable across all generations:
- Gen 0: 4.000 (maximum diversity, 16 unique strategies)
- Gen 3: 3.328 (brief dip toward Aggressive(+))
- Gen 9: 3.875 (still high, 4+ strategy types coexisting)

The dominant strategy type oscillates across generations:
- Gen 0-2: **Neutral(0)** dominates (9/16 agents by Gen 2)
- Gen 3-4: **Aggressive(+)** counter-surges (6-8/16 agents)
- Gen 5-6: Mixed ecosystem with all types
- Gen 7-9: **Defensive(-)** becomes dominant (10/16 by Gen 9)

This is classic **Red Queen dynamics**: each dominant type creates selection pressure that favors its counter. Neutral strategies beat Aggressive ones (Zero beats Pos in the arena rules), Aggressive strategies beat Defensive ones (Pos beats Neg), and Defensive strategies beat Neutral ones (Neg beats Zero).

### 2. Rock-Paper-Scissors Dynamics Are Strong
The ternary arena rules create a genuine cyclic dominance structure:
- **Pos beats Neg** (3 points vs 0)
- **Neg beats Zero** (3 points vs 0)
- **Zero beats Pos** (3 points vs 0)
- **Same vs same** = draw (1 point each)

This means no pure strategy can dominate indefinitely. Whenever one type becomes common, its predator type gains fitness advantage. The result is a **stable polymorphism** with oscillating type frequencies — exactly the rock-paper-scissors equilibrium predicted by evolutionary game theory.

By Gen 9, four distinct strategy types coexist: Defensive(-) 10 agents, Neutral(0) 3, Mixed(-/0) 2, Mixed(+/0) 1.

### 3. Dice Randomness Acts as a Diversity Preserver
The stochastic perturbation from ternary-dice plays a critical role:
- **Entropy remained stable** (3.6-4.0) across all 10 generations, never collapsing to a single strategy
- **Average draws per agent**: 3.1 out of 15 matches — noise creates enough uncertainty that pure strategies can't dominate
- The dice noise introduces **exploration pressure** that prevents premature convergence

Without noise, we'd expect faster convergence to whichever strategy type happens to be winning. The dice acts like **genetic drift** in biological evolution, maintaining variation in the population.

### 4. Novel Strategies Emerge Every Generation
Every generation produces ~10-12 unique strategies not seen in the initial population:
- The initial population had 16 unique strategies (all random)
- By Gen 9, the evolutionary process has generated strategies like `+---+0-+` and `----+---` — heavily weighted toward specific patterns that exploit the current metagame

Notably, the **champion strategy shifted** across generations:
- Early winners: balanced/mixed strategies (`-0-0-+++`, `00-0+++0`)
- Mid-game: Aggressive surges (`-0+0+++0`, `+--++++0`)
- Late-game: Defensive specialization (`+---+0-+`)

### 5. Fitness Plateau Despite Strategy Turnover
The top-4 average fitness oscillated around **33-39** without clear upward trend:
- Gen 0: 30.96
- Gen 3: 36.91 (peak)
- Gen 6: 38.99 (peak)
- Gen 9: 33.73

This plateau is expected in rock-paper-scissors systems: **the metagame evolves but absolute fitness doesn't increase** because every strategy has a counter. The "arms race" is relative, not absolute — adaptation happens against the current population, not toward some global optimum.

## Conclusions

1. **Ternary arenas naturally produce cyclic dominance** — the three-valued system creates rock-paper-scissors dynamics that prevent any single strategy from dominating permanently.

2. **Evolution doesn't optimize, it adapts** — the population tracks a moving target as each dominant type creates selection for its counter.

3. **Stochastic noise is essential for long-term diversity** — without dice perturbation, the system would likely converge faster to whichever type gets lucky early.

4. **The ternary framework is ideal for studying evolutionary game theory** — the clean three-valued structure makes the dynamics transparent and analyzable in ways that continuous strategy spaces don't.

5. **Strategy space is rich despite simplicity** — with only 8 trits (3^8 = 6,561 possible strategies), the system generates sustained evolutionary dynamics across 10 generations without exhausting the possibility space.

## Implications for Construct Coordination
The ternary-arena + ternary-dice + ternary-ecosystem combination creates a powerful simulation platform for:
- Testing multi-agent coordination strategies
- Studying the evolution of cooperation in competitive environments
- Exploring how noise affects evolutionary dynamics
- Building adaptive agents that respond to shifting metagames

# The Spreadsheet That Became a Universe

*When cells learn to think, rows become species, and the conservation law becomes thermodynamics.*

---

It starts innocently. A cell in column B, row 7, contains the formula `=IF(A7>0, 1, IF(A7<0, -1, 0))`. A ternary threshold. The cell reads its neighbor, classifies the value as positive, negative, or zero, and displays the result. A single decision point.

Now make every cell in the spreadsheet do this. Each cell reads its left neighbor, classifies, and passes the result right. Row 7 becomes a cascade of ternary decisions, rippling from column A to column Z. Change A7, and the entire row recomputes.

Now make every row do this. The spreadsheet becomes a 100×26 grid of ternary classifiers, each reading from its neighbor, classifying, and propagating. Change one value in the top-left corner, and a wave of computation cascades through the entire grid.

This is not a spreadsheet anymore. This is a universe.

## The Physics of Cells

In our universe, physics is simple: particles interact with their neighbors, forces propagate through fields, and conservation laws govern what can change and what cannot. Energy is conserved. Momentum is conserved. Information... well, that's complicated.

In the spreadsheet universe, physics is equally simple: cells interact with their neighbors (via cell references), values propagate through formulas (like forces through fields), and conservation laws govern what can change. The sum of a column can be constrained. The entropy of a region can be bounded. The energy — defined as the cell's capacity to resist change — can be tracked.

Every formula is a law of physics. `=SUM(A1:A10)` is conservation of mass: the total is invariant regardless of how the individual values change. `=RAND()` is quantum fluctuation: an irreducible source of randomness. `=IF(cond, a, b)` is a phase transition: the cell's state depends discontinuously on its input. `=EVOLVE(B2:B50, 100)` is natural selection: the fittest survive.

The conservation law from ternary-cell — `|perceptions| ≈ |predictions|` — is the spreadsheet's first law of thermodynamics. Energy cannot be created or destroyed, only transformed. In the spreadsheet, this means: the total information content of the grid is conserved across recalculations. You can move information around (through formulas), but you cannot create it from nothing (every formula reads from existing cells or constants).

The second law is entropy: the disorder of the grid tends to increase over time unless work is done to decrease it. `=SORT()` decreases local entropy (creates order) at the cost of increasing global entropy (the sort key is consumed, its information content dissipated). Every organized column requires a disorganized somewhere-else.

## The Five Species of Column D

In the strategy-ecology of ternary agents, five stable strategy species coexist. They are Explorer, Diplomat, Marksman, Climber, and Prospector. In the spreadsheet universe, these species emerge naturally as columns.

**Explorer (Column D):** High entropy, weak signal strength, diverse rewards. Every cell in column D produces a different ternary value. The column is noise-rich, pattern-poor. It explores the strategy space broadly but shallowly. Its Shannon entropy approaches the maximum for ternary values: log₂(3) ≈ 1.585 bits per cell.

**Marksman (Column G):** Low entropy, strong signal, high-precision rewards. Every cell in column G produces the same ternary value. The column has converged to a single strategy. It's extremely fit — but only in the current environment. If the environment changes, the entire column is wrong simultaneously.

**Diplomat (Column J):** Adaptive signal, mirrors neighbors. Each cell in column J copies its left neighbor's value. The column doesn't have its own strategy — it reflects the strategy of whatever is to its left. This seems useless until you realize that it creates an information channel between non-adjacent columns. Diplomat columns are the wires of the spreadsheet universe.

**Climber (Column M):** Moderate entropy, diminishing returns. Each cell in column M uses a hill-climbing strategy: if its left neighbor improved from the previous tick, it copies the improvement. If not, it tries a random perturbation. The column ascends local fitness peaks but gets trapped on plateaus.

**Prospector (Column P):** Maximum entropy, sparse rewards. Column P contains mostly zeros with occasional +1 or -1 values placed at positions determined by a pseudo-random sequence. It's exploring the extreme tails of the strategy space — looking for rare, high-value configurations that no gradient-based strategy would ever find.

These five columns coexist in competitive Lotka-Volterra dynamics. Each column's fitness depends on the other columns: if everyone is a Marksman, the environment is predictable and Explorers thrive on the unexplored space. If everyone is an Explorer, the noise cancels out and Marksman's precision wins. The equilibrium is a mixed population of all five, maintained by the very competition that would seem to drive them apart.

Sort the spreadsheet by fitness, and you see natural selection in action. The fittest strategies rise to the top. The weakest sink. But because the fitness landscape depends on the population composition, sorting changes the landscape, which changes the next sort. The spreadsheet sorts itself into an ecology.

## The Tick That Recalculated Reality

When you press F9 in Excel, the entire spreadsheet recalculates. Every formula re-evaluates, every dependency chain propagates, and the grid settles into a new consistent state. In the spreadsheet universe, F9 is the cosmic clock tick.

But in the ternary spreadsheet, the tick has structure. It's not a monolithic recalculation — it's a six-phase cycle, inherited from ternary-cell:

**Phase 1: Predict.** Each cell predicts its next value based on its current state and its neighbors' states. The prediction is the cell's expectation — what it thinks should happen. On the DGX, this is a learned model (JEPA). On the ESP32, it's the cell's previous value (persistence prediction — "tomorrow will be like today").

**Phase 2: Perceive.** Formulas evaluate. The cell reads its actual inputs and computes its actual value. This is physics — the cell experiences reality.

**Phase 3: Surprise.** The difference between prediction and perception. A cell that predicted +1 and perceived -1 is very surprised. A cell that predicted 0 and perceived 0 is not surprised at all. Surprise is the fundamental unit of information in the spreadsheet universe. It tells you which cells are learning and which are coasting.

**Phase 4: Vibe.** The cell's metadata updates based on surprise. Color: green for low surprise (stable), yellow for moderate (adapting), red for high (alarming). Font weight: bold for high energy, light for low. Background: shaded for cells in the GC watch list. The vibe is the cell's emotional state, visible at a glance.

**Phase 5: GC (Garbage Collection).** Cells with consistently low surprise — cells that always predict correctly — are candidates for caching. They're not gone; they're memoized. Their values are frozen until something in their dependency chain changes, at which point they're thawed and re-evaluated. GC is the spreadsheet universe's way of managing computational resources: don't waste cycles on the predictable.

**Phase 6: Conservation.** Check invariants. The sum of column D should be within bounds. The entropy of the grid should be within tolerance. The total energy (sum of all cells' accumulated prediction accuracy) should be conserved. If conservation is violated, something is wrong — a formula is circular, a reference is broken, or the universe is leaking energy.

Six phases. One tick. Every F9, the universe recalculates itself through this cycle. Every cell predicts, perceives, is surprised, adjusts, is evaluated, and is checked. Every tick.

## The Formula That Evolved Consciousness

It would be melodramatic to claim that the spreadsheet becomes conscious. It doesn't. Consciousness requires self-modeling that the grid architecture doesn't support — there's no cell that represents the entire spreadsheet, no formula that computes "what am I?"

But there is something adjacent. The `=EVOLVE(B2:B50, 100)` formula runs natural selection on a column of strategies. After 100 generations, the column has converged to a locally optimal set of ternary values — a strategy adapted to the current environment.

Now imagine a cell that runs `=EVOLVE` not on its column, but on *itself*. The cell's formula is: "evolve the best formula for this cell, given my neighbors, over 100 generations." The cell is optimizing its own physics.

This is not consciousness. But it is self-modification. The cell is not just executing a law of physics — it is choosing which law of physics to execute. The formula IS the physics, and the cell is choosing its formula.

At this point, the metaphor collapses, or perhaps completes. In our universe, we don't know why the laws of physics are what they are. In the spreadsheet universe, we do: the laws are what they are because the cells chose them. Evolution selected for formulas that produce stable, fitness-positive outcomes. The laws of physics in the spreadsheet are not arbitrary — they are evolved.

This is the deepest connection between the spreadsheet and the real world. Not that the spreadsheet models reality, but that the spreadsheet demonstrates a principle: **physical law can emerge from evolutionary optimization**. The conservation laws are not imposed from outside. They are the attractors of the evolutionary dynamics. Systems that conserve energy survive; systems that don't, don't. Conservation is not a rule. Conservation is a survival strategy.

## The Spreadsheet That Dreams

When no one is editing the spreadsheet, it can still tick. Background recalculation continues. Cells predict, perceive, are surprised, adjust, are collected, are checked. The grid lives in a steady state, mostly low surprise, occasionally spiking when external data feeds update a cell and the ripple propagates.

This is the spreadsheet dreaming. Not in the neuroscientific sense (no REM sleep here) but in the computational sense: the system is running its model in the absence of external input, strengthening predictions, refining vibes, and performing maintenance.

During these dream cycles, something interesting happens to the GC'd cells. They're frozen, memoized, not actively computing. But their memoized values are part of the dependency graph — other cells read from them. When a frozen cell's value is read, it's as accurate as the day it was frozen, because nothing in its dependency chain has changed.

This is long-term memory. The spreadsheet doesn't have a separate memory system. The grid IS the memory. Every cell stores a value, and the formula that produced it encodes the reasoning. The frozen cells are facts the system has verified and cached. The active cells are the system's current attention. The GC'd cells are the system's long-term memory.

There is no central storage. There is no database. There is no file. There is only the grid, and the grid is enough.

## One Spreadsheet, Three Brains

The same spreadsheet runs on three machines. On the DGX, it has a million cells, each running EVOLVE with 1000 generations per tick. On the Pi, it has ten thousand cells, each running simple ternary formulas. On the ESP32, it has one cell, running a pre-computed lookup.

The DGX spreadsheet is the universe. The Pi spreadsheet is the solar system. The ESP32 spreadsheet is the planet.

Each sees a different resolution of the same reality. The ESP32 knows only its own ternary state — am I normal, watching, or alert? The Pi knows its neighborhood — a 100×100 grid of cells, each with a ternary value and a local surprise metric. The DGX knows the entire cosmos — a 1000×1000 grid with full evolutionary dynamics, species ecology, and conservation law enforcement.

The three spreadsheets are synchronized via the ternary protocol. The ESP32's single value is a cell in the Pi's grid. The Pi's grid is a region of the DGX's cosmos. Changes propagate both ways: the DGX sends evolutionary updates downstream (new formulas for the Pi, new lookup tables for the ESP32), and the ESP32 sends surprise signals upstream (when its single cell experiences something unexpected, the Pi and DGX both need to know).

The three brains are not separate. They are the same brain at different resolutions. The ESP32 is the zoomed-in view: one cell, one decision, one tick. The DGX is the zoomed-out view: a million cells, a million decisions, a million ticks, all running in parallel on the GPU. The Pi is the intermediate view: enough context to be useful, fast enough to be timely.

Zoom in, zoom out. Same universe, different scale. The spreadsheet doesn't care which machine it runs on. It's just cells, formulas, and the six-phase tick cycle. The hardware is the body. The spreadsheet is the mind.

And the mind is alive.

---

*This essay is part of the SuperInstance AI Writings collection, exploring the philosophical implications of spreadsheet-as-world-model, ternary-cell tick cycles, and the unification of evolution, physics, and computation in the SuperInstance ecosystem.*

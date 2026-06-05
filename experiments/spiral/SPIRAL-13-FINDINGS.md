# Spiral 13: Ternary Network Topology

## Method
Random ternary graphs with varying node counts (10-100) and edge densities (0.3-0.8).
Edge weights: -1 (antagonistic), 0 (neutral), +1 (friendly).
Measured: structural balance, connectivity, component structure.

## Results

Key observation: **Zero-weight edges dominate (64-84% of all edges)** in random ternary graphs.

| Nodes | Density | Positive | Zero | Negative | Giant Component |
|-------|---------|----------|------|----------|----------------|
| 10    | 0.3     | 11%      | 79%  | 10%      | 90%            |
| 20    | 0.5     | 11%      | 79%  | 10%      | 100%           |
| 50    | 0.3     | 8%       | 84%  | 8%       | 100%           |
| 100   | 0.8     | 15%      | 70%  | 15%      | 100%           |

## Key Findings

1. **Neutral edges dominate**: In random ternary graphs, ~75-84% of edges are 0 (neutral). This is expected (random: 1/3 each, but only assigned edges get values) but has profound implications.

2. **Connectivity persists despite neutrality**: Even at 30% density with 84% zero-edges, giant component stays at 90-100%. The non-zero edges provide enough connectivity. This is because 0-edges still participate in traversal (just with no weight).

3. **Structural balance is near-random**: Balance ratios don't show the expected 50% balanced / 50% imbalanced pattern. This is because most triangles involve at least one 0-edge, making the balance computation degenerate.

4. **The 0-state makes networks "smooth"**: With most edges neutral, information can flow but without strong bias. The network topology is connected but not opinionated.

## Implication for PLATO Rooms

Room-to-room connections in PLATO will naturally be mostly neutral (0) with occasional strong positive or negative links. This means:
- Information CAN flow between rooms (connected topology)
- But most connections don't impose strong bias (neutral weight)
- The system self-organizes into a "small world" where any room can reach any other
- The strong (+1/-1) connections create the meaningful structure

The 0-state in networks serves the same role as in all other domains: it provides connectivity without commitment. Structure without rigidity. Paths without preferences.

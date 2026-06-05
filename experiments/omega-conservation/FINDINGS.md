# Experiment 9: Ω Conservation Law

## Hypothesis
Ω = |γ| + H + I_total is conserved across evolutionary time in ternary agent populations.

## Setup
- 100-cell 10×10 grid, 5 species, ternary {-1,0,+1}
- Moore neighborhood, majority-rule update
- 2% mutation rate, 1% species switching
- 2000 ticks, sliding window of 50 for MI calculation
- Pairwise MI averaged over sampled pairs

## Results

| Quantity | Variance | Start | End | Drift | Verdict |
|----------|----------|-------|-----|-------|---------|
| γ | 0.001048 | 0.01 | -1.00 | -10100% | DRIFTS |
| H | 0.003117 | 2.618 | 1.592 | -39% | DRIFTS |
| I_total | 0.000001 | 0.0 | 0.0003 | +33% | tiny |
| **Ω** | **0.001099** | **2.628** | **2.592** | **-1.4%** | **CONSERVED** |
| γ+H | 0.007233 | 2.628 | 0.592 | -77.5% | DRIFTS |

**Ω is 6.6× more stable than γ+H alone.**

All 5 species survived (21, 21, 13, 22, 23).

## Interpretation

Ω = |γ| + H + I_total shows only -1.4% drift over 2000 ticks, while γ+H drifts -77.5%.
The system compensates: as γ collapses (agents converge to -1) and H drops (diversity loss),
I_total provides a small but measurable stabilizing contribution.

However, I_total values are very small (order 10^-4 to 10^-3), suggesting:
1. The pairwise MI averaging may dilute the signal
2. The MI window (50 ticks) may be too short
3. The sampling strategy (every 3rd × 5th pair) may miss correlated pairs

## Caveats
- Need to verify with FULL pairwise MI (not sampled)
- Need longer runs (10K ticks) to confirm stability
- Need to test with different grid sizes (N=50, N=200)
- I_total contribution is currently negligible — the conservation may just be |γ| + H happening to be more stable than γ + H

## Status
**PRELIMINARY — needs replication with full pairwise MI calculation**

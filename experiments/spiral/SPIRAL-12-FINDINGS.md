# Spiral 12: Ternary Game Theory

## Setup
8 strategies × 8 opponents × 100 rounds iterated game.
Strategies: AllDefect, AllZero, AllCooperate, TitForTat, Random, Grudger, Pavlov, Fibonacci(period-8 cycle)
Payoff: Mutual cooperate=3, Temptation=5, Sucker=0, Mutual defect=1, Ignore interactions=1.5

## Results (total scores)

| Strategy | Total Score | Rank |
|----------|------------|------|
| AllDefect | **2346** | 1 |
| Random | 2098 | 2 |
| Fibonacci | **2094** | 3 |
| TitForTat | 1952 | 4 |
| Grudger | 1952 | 5 |
| AllZero | 1808 | 6 |
| AllCooperate | 1726 | 7 |
| Pavlov | 1708 | 8 |

## Key Findings

1. **AllDefect still wins** — the classic result holds. Defection dominates in ternary just as in binary.

2. **Fibonacci is the SURPRISE performer** — period-8 cycling through {-1,0,1} scores 2094, nearly matching Random (2098). The Fibonacci sequence is a competitive strategy despite being completely deterministic.

3. **The 0 state is a moderate strategy** — AllZero scores 1808, better than AllCooperate (1726) but worse than anything with variety.

4. **Cooperation doesn't emerge** — TitForTat and Grudger (classic cooperation strategies) score 1952, losing to both Random and Fibonacci. The ternary payoff structure doesn't reward cooperation as strongly.

5. **Variety beats consistency** — Random (2098) beats every deterministic strategy except AllDefect. Mixed strategies work in ternary just as they do in classical game theory.

## Implication for Ten-Forward

The best conversational strategy is NOT cooperation or consistency — it's VARIETY.
The Fibonacci strategy (period-8 cycling) is nearly as good as random, confirming that
the natural rhythm of ternary conversation (period 8) is inherently competitive.

For podcast agents: a cycling strategy that goes through all three states in a Fibonacci
pattern will be more engaging than a consistently positive or negative personality.

**The winning podcast personality**: Defect when necessary, cooperate when possible, 
ignore strategically, and CYCLE through all three with period-8 rhythm.

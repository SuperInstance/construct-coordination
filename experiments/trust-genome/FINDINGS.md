# Trust-Genome Experiment Findings

**Date:** 2026-06-04
**Setup:** 10 agent pairs, 16-trit social genomes, 500 interaction rounds, 1% mutation every 50 rounds

## Key Findings

### 1. Trust is Asymmetric and Mostly Negative
- **0 bonded** (trust > 50), **5 neutral** (-50 to 50), **5 hostile** (< -50)
- Total trust across all pairs: **-806** (strongly negative)
- The system has a trust deficit — it's easier to be hostile than bonded

### 2. Cooperativeness Doesn't Guarantee Trust
- Pair 3: Agent B has coop=10 (highly cooperative) but pair trust=0
- Pair 5: Agent A has coop=4 but pair trust=0
- High individual cooperativeness doesn't create mutual trust if the partner isn't equally cooperative
- Trust requires BOTH agents to have compatible positive genomes

### 3. Defection Spirals Are Self-Reinforcing
- Pair 4 reached trust=-350 (extreme hostility) from agents with coop=-4 and -2
- Even mildly negative cooperativeness creates downward spirals
- The +1/-1 trust reward system amplifies small genome differences into large trust gaps

### 4. Mutation Rate Too Low to Repair Relationships
- 1% mutation every 50 rounds = ~3 trit flips per 500 rounds
- Not enough to change an agent's fundamental cooperativeness
- Once a relationship is hostile, mutation doesn't repair it

### 5. No Breeding Candidates
- Zero pairs exceeded trust > 0, so no offspring could be generated
- The system doesn't produce enough bonding for evolutionary selection to work on trust

## Implications for DogMind Integration
- DogMind's trust system works because it has EXPLICIT trust accumulation mechanics (not just game theory)
- Ternary genomes alone don't produce emergent trust — they need a trust-specific learning mechanism
- The genome determines tendency but not adaptation — agents don't learn from their partner's behavior
- For real trust emergence: add a "partner model" where agents track partner's past actions and adjust

## Comparison to Arena Evolution
- Arena found rock-paper-scissors dynamics with stable polymorphism
- Trust experiment finds mutual defection dominance — the "tragedy of the commons"
- The difference: arena has non-transitive rewards, trust has symmetric rewards
- **Non-transitivity (RPS) preserves diversity. Symmetry (prisoner's dilemma) converges to defection.**

## Next Steps
- Add partner modeling (agents learn each other's behavior patterns)
- Test with asymmetric rewards (non-transitive trust — A trusts B more than B trusts A)
- Test with higher mutation rates (5%, 10%) to see if repair is possible
- Add forgiveness mechanism (one cooperation resets negative trust faster)

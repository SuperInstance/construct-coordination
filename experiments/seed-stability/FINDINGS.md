# Seed Stability Findings

**Date:** 2026-06-04  
**Setup:** 10 base seeds × 64 trit entries each, ternary-seed/ternary-genome/ternary-dice/ternary-arena crates

---

## 1. Mutation Stability: Linear Degradation, No Phase Transition

| Mutation Rate | Avg Hamming Distance | Inference Changes |
|---------------|---------------------|-------------------|
| 1%            | 0.64                | 0.64              |
| 5%            | 3.20                | 3.20              |
| 10%           | 6.40                | 6.40              |
| 20%           | 12.80               | 12.80             |
| 50%           | 32.00               | 32.00             |

**Finding:** Mutation damage scales **perfectly linearly** with mutation rate. There is no critical threshold, no phase transition, no "breaking point." At 1% mutation, <1 trit flips. At 50%, half the seed flips. Every flipped trit changes inference output 1:1.

**Verdict:** Seeds are **fragile in proportion to mutation**. The `SeedMutator` uses deterministic modular thresholds (`threshold % 100 < rate × 100`), which creates this clean linear relationship. For SMP to work, mutation rates must be kept low (1-5%) or the seed becomes a different seed entirely. This is expected for a compact encoding — there's no redundancy to absorb mutations.

**Implication for SMP:** Seeds lack error correction. Real DNA has degenerate codons and introns that buffer mutations. SMP seeds have neither. Adding redundancy (parity trits, error-correcting codes) would be needed for robust evolutionary search.

---

## 2. Conservation Law: Universally Violated

All 10 base seeds and all mutations showed **0% conservation compliance**.

The SMP spec defines conservation as: `γ + H ≈ 1.283 - 0.159 × ln(V)`  
For V=64 entries: target ≈ 0.622  
Actual values: γ + H ≈ 1.9 (consistently)

**Finding:** Random ternary distributions with ~⅓ Neg have γ ≈ 0.35 and Shannon entropy H ≈ 1.55, giving γ + H ≈ 1.9. The conservation target of 0.622 is only achievable with very low entropy (near-uniform seeds) or very high γ (mostly suppressed outputs). The seeds produced by `SeedEncoder` from random observations fundamentally don't satisfy the conservation law.

**Root cause:** The conservation formula in the spec assumes the seed acts as a *weight mask on an output distribution*, where γ is the fraction of suppressed outputs. But the current `Seed` struct stores (input_hash, trit) *observations*, not output distribution weights. The formula doesn't apply to this data structure. The spec describes a different seed format (strategy_vector + ternary_weights) than what `ternary-seed` actually implements.

**Implication for SMP:** The `ternary-seed` crate and the SMP spec are **misaligned**. The crate implements a key-value observation store. The spec describes a behavioral mask. These are conceptually related but structurally different. Conservation validation needs the spec's binary format (strategy_vector + ternary_weights), not the crate's current `Vec<(u64, Trit)>`.

---

## 3. Cross-Breeding: No Overlap, No Interaction

All cross-breeding pairs produced offspring with 128 entries (64+64) and **0 shared keys**. The `SeedCombiner`'s overlap resolution (majority rule → Zero for conflicts) was never triggered.

**Finding:** Seeds generated with unique input hashes don't share any entries. The combiner simply concatenates them. There is no actual genetic recombination happening — it's additive, not integrative.

**Implication for SMP:** For cross-breeding to work meaningfully, seeds need **overlapping key spaces**. Two seeds encoding behavior for the *same* set of input hashes would produce conflicts that the combiner could resolve. This would require:
1. A canonical key space (fixed input hashes that all seeds address)
2. Seeds that encode dispositions over that shared space

Without shared keys, cross-breeding is just union, not breeding.

---

## 4. Tournament: Pos-Dominance Under RPS Rules

Under the arena's rock-paper-scissors rules (Pos > Neg, Neg > Zero, Zero > Pos):

- **Top performers:** Agents 2 and 8 (Pos ratio ≈ 0.44, Neg ratio 0.22-0.31)
- **Champions varied across runs:** Agents 1, 2, 4, 13 each won in different bracket shuffles
- **Pos-heavy strategies consistently score higher** because Pos beats Neg directly

**Finding:** Tournament outcomes depend heavily on **bracket position** (who you face), not just strategy quality. The same agent wins in one bracket and loses in another. Pos-biased strategies have an advantage only because the RPS rules favor Pos in a pool where many agents are Neg-heavy.

**The deeper issue:** The arena rules create a non-transitive game (A beats B, B beats C, C beats A). In such games, there is no "best" strategy — only strategies that exploit the current population. This is actually *correct* for an evolutionary system, but it means tournament winners aren't "better seeds" — they're better-matched seeds.

**Trait dominance:** No trait is universally dominant. Survival depends on the competitive environment. This is ecologically accurate but makes "evolving better seeds" through tournament selection non-trivial.

---

## 5. Genome Evolution: Rapid Convergence to Homogeneity

Starting from random ternary genomes (20 agents × 32 genes):
- **Generation 0:** Avg fitness -1.4, alleles distributed ~⅓ each
- **Generation 20:** Avg fitness 14.0, majority Pos
- **Generation 49:** **100% uniform** — all 20 agents identical (23 Pos, 9 Zero, 0 Neg)

**Finding:** Selection pressure toward fitness=sum(alleles) causes total convergence in ~20 generations. The mutation rate self-adapts down to 0.1% as fitness improves, accelerating convergence. By gen 49, **all diversity is lost**.

**This is a textbook genetic algorithm failure mode.** The fitness function (sum of trits = count Pos - count Neg) is too simple and has a trivial optimum (all Pos). There's no fitness landscape complexity, no local optima, no epistasis between genes.

**Implication for SMP:** If seed fitness is measured by a single scalar, evolutionary search will converge to a single optimum and lose all diversity. Meaningful seed evolution needs:
1. **Multi-objective fitness** (e.g., conservation compliance + behavioral diversity + task performance)
2. **Niching** (reward different behavioral profiles, not just "best" one)
3. **Epistatic genes** (gene combinations matter, not just individual alleles)

---

## Overall Assessment: Does SMP Produce Stable Inference?

### What Works
- **Deterministic encoding/decoding:** Same seed always produces same output. ✓
- **Controlled mutation:** Mutation rate directly controls how much the seed changes. ✓
- **Reproducible:** All results are deterministic given the same PRNG seeds. ✓

### What Doesn't Work (Yet)
- **Conservation compliance:** The crate and spec are misaligned. Conservation can't be validated on the current data structure.
- **Error tolerance:** Seeds are fragile. No redundancy, no error correction. 5% mutation = ~3 trit flips = 3 changed outputs. For a 64-entry seed, that's ~5% behavioral change with no way to recover.
- **Cross-breeding:** Without shared key spaces, combining seeds is concatenation, not recombination.
- **Evolutionary stability:** Selection pressure collapses diversity in ~20 generations. Tournament winners depend on bracket luck, not intrinsic quality.

### Critical Gap
The `ternary-seed` crate implements a **key-value observation store**. The SMP spec describes a **behavioral mask** with strategy vectors and ternary weights applied to model output distributions. These are related concepts but different implementations. The spec's conservation law, species classification, and distillation process all assume the spec format. The crate doesn't implement that format.

### Recommendations
1. **Implement the spec's binary format** in `ternary-seed` (strategy_vector + ternary_weights + conservation_params)
2. **Add error-correcting codes** (at minimum, parity trits) for mutation resilience  
3. **Define canonical key spaces** for cross-domain seeds to enable real recombination
4. **Use multi-objective fitness** for evolutionary search (not just sum-of-trits)
5. **Add niching or speciation** to prevent convergence to a single optimum

---

*Experiment code: `/home/phoenix/repos/construct-coordination/experiments/seed-stability/`*

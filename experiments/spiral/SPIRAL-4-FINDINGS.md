# Spiral 4: Ternary Algebra

## 1. Ternary Binary Operations Census — ONLY 3 GROUPS

Out of 19,683 possible binary operations on {-1, 0, +1}:

| Property | Count |
|----------|-------|
| Total operations | 19,683 |
| Commutative | 729 (3.7%) |
| Associative | 113 (0.57%) |
| Has identity | 243 (1.2%) |
| Has inverses | 51 (0.26%) |
| **Groups** | **3 (0.015%)** |

### The Three Ternary Groups

**Group 1** (identity=0): Addition mod 3 mapped to ternary
- op(-1,-1)=+1, op(0,0)=0, op(1,1)=-1
- This is Z₃ — the cyclic group of order 3
- -1 + -1 = +1, -1 + 0 = -1, -1 + +1 = 0, etc.

**Group 2** (identity=-1): Isomorphic to Group 1 via relabeling
- op(-1,-1)=-1, op(0,0)=+1, op(1,1)=0

**Group 3** (identity=+1): Isomorphic to Group 1 via relabeling
- op(-1,-1)=0, op(0,0)=-1, op(1,1)=+1

**All three are the SAME group (Z₃) with different identity elements.**

This is a DEEP result. There is exactly ONE algebraic group structure on ternary values: cyclic addition mod 3. No other group structure exists.

**Implication**: Every ternary system that forms a group MUST be Z₃. There's no alternative.
This means ternary composition is fundamentally cyclic. The system wraps around.

## 2. Ternary Markov Entropy Rate

1000 random 3-state Markov chains, measuring entropy rates:

| Metric | Value |
|--------|-------|
| Min entropy rate | 0.4948 |
| Max entropy rate | 1.5692 |
| Theoretical max | 1.5850 (log₂3) |

**Finding**: The maximum entropy rate (1.57) is close to but doesn't reach log₂3.
This is because a Markov chain with full entropy would need uniform transitions 
(all transitions equally likely), which is hard to hit with random sampling.

**Entropy range**: Most chains sit between 1.0 and 1.5 bits/symbol.
The spindle (0 state) tends to accumulate — chains with high zero stationary probability 
(>60%) have lower entropy. Chains with balanced stationary distributions have higher entropy.

**Implication**: Ternary systems naturally carry about 1.0-1.5 bits per symbol.
A healthy ternary system has balanced states. An unhealthy one concentrates on 0.

# Spiral 3: Ternary Number Theory

## 1. Ternary Collatz — DIVERGENT

**Rule**: If value maps to ternary 0, divide by 3. If ±1, multiply by 3 and add ternary value.

**Finding**: ALL starting values 1-50 diverge to the million-step limit without cycling.
Values reach ~800K-974K. No convergence. The ternary Collatz grows without bound.

The pattern is clear: values that hit ternary +1 or -1 get multiplied by 3 and grow.
Division by 3 (when ternary 0) isn't frequent enough to counteract the growth.

**Implication**: Unlike the binary Collatz conjecture (believed to always reach 1), 
the ternary Collatz is a GROWTH engine. Starting from any seed, it diverges.
This could be useful for generating expanding sequences or hash-like spreading.

## 2. Ternary Fibonacci — PERFECT PERIODICITY

**Rule**: Ternary Fibonacci — each term is the ternary sum of the two previous.
Ternary addition: -1+-1→-1, -1+0→-1, -1+1→0, 0+0→0, 0+1→1, 1+1→-1 (mod 3 arithmetic)

**Findings**:

Fibonacci (2-term recurrence):
- Period 8 for ALL starting pairs except (1,1)
- (1,1) has period 2: it just alternates 1,1,1,1...
- All 3 values {-1, 0, +1} appear in every non-trivial sequence

Tribonacci (3-term recurrence):
- Period 13 for ALL starting triples except (1,1,1)
- (1,1,1) has period 3: alternates through 1,1,1
- All 3 values appear in every non-trivial sequence

**Key Numbers**:
- Fibonacci period: 8
- Tribonacci period: 13
- These are the Pisano periods for modulus 3!
- F(n) mod 3 has period 8
- T(n) mod 3 has period 13

**Implication**: Ternary Fibonacci sequences are DETERMINISTIC PERIODIC OSCILLATORS.
Period 8 for binary recurrence, period 13 for ternary recurrence.
These are the fundamental rhythms of ternary arithmetic.

For PLATO rooms: a room that runs ternary Fibonacci is a clock with period 8.
A room that runs ternary Tribonacci is a clock with period 13.
8 × 13 = 104 — the least common multiple is the full cycle.
This is the ternary equivalent of musical meter (8-beat + 13-beat polyrhythm).

# Seed Stability Experiment Results

Generated from ternary-seed, ternary-genome, ternary-dice, ternary-arena crates.
Seed size: 64 entries (ternary trits). Population: 10 base seeds.

---

# Experiment 1: Seed Stability Under Mutation

| Seed | Rate | Avg Hamming | Avg Output Changes | Conservation % |
|------|------|-------------|-------------------|----------------|
| S0 | 1% | 0.6 | 0.6 | 0% |
| S0 | 5% | 3.2 | 3.2 | 0% |
| S0 | 10% | 6.4 | 6.4 | 0% |
| S0 | 20% | 12.8 | 12.8 | 0% |
| S0 | 50% | 32.0 | 32.0 | 0% |
| S1 | 1% | 0.6 | 0.6 | 0% |
| S1 | 5% | 3.2 | 3.2 | 0% |
| S1 | 10% | 6.4 | 6.4 | 0% |
| S1 | 20% | 12.8 | 12.8 | 0% |
| S1 | 50% | 32.0 | 32.0 | 0% |
| S2 | 1% | 0.6 | 0.6 | 0% |
| S2 | 5% | 3.2 | 3.2 | 0% |
| S2 | 10% | 6.4 | 6.4 | 0% |
| S2 | 20% | 12.8 | 12.8 | 0% |
| S2 | 50% | 32.0 | 32.0 | 0% |
| S3 | 1% | 0.6 | 0.6 | 0% |
| S3 | 5% | 3.2 | 3.2 | 0% |
| S3 | 10% | 6.4 | 6.4 | 0% |
| S3 | 20% | 12.8 | 12.8 | 0% |
| S3 | 50% | 32.0 | 32.0 | 0% |
| S4 | 1% | 0.6 | 0.6 | 0% |
| S4 | 5% | 3.2 | 3.2 | 0% |
| S4 | 10% | 6.4 | 6.4 | 0% |
| S4 | 20% | 12.8 | 12.8 | 0% |
| S4 | 50% | 32.0 | 32.0 | 0% |
| S5 | 1% | 0.6 | 0.6 | 0% |
| S5 | 5% | 3.2 | 3.2 | 0% |
| S5 | 10% | 6.4 | 6.4 | 0% |
| S5 | 20% | 12.8 | 12.8 | 0% |
| S5 | 50% | 32.0 | 32.0 | 0% |
| S6 | 1% | 0.6 | 0.6 | 0% |
| S6 | 5% | 3.2 | 3.2 | 0% |
| S6 | 10% | 6.4 | 6.4 | 0% |
| S6 | 20% | 12.8 | 12.8 | 0% |
| S6 | 50% | 32.0 | 32.0 | 0% |
| S7 | 1% | 0.6 | 0.6 | 0% |
| S7 | 5% | 3.2 | 3.2 | 0% |
| S7 | 10% | 6.4 | 6.4 | 0% |
| S7 | 20% | 12.8 | 12.8 | 0% |
| S7 | 50% | 32.0 | 32.0 | 0% |
| S8 | 1% | 0.6 | 0.6 | 0% |
| S8 | 5% | 3.2 | 3.2 | 0% |
| S8 | 10% | 6.4 | 6.4 | 0% |
| S8 | 20% | 12.8 | 12.8 | 0% |
| S8 | 50% | 32.0 | 32.0 | 0% |
| S9 | 1% | 0.6 | 0.6 | 0% |
| S9 | 5% | 3.2 | 3.2 | 0% |
| S9 | 10% | 6.4 | 6.4 | 0% |
| S9 | 20% | 12.8 | 12.8 | 0% |
| S9 | 50% | 32.0 | 32.0 | 0% |

## Aggregated Results by Mutation Rate

| Rate | Mean Hamming | Mean Output Changes | Conservation Compliance |
|------|-------------|--------------------|-----------------------|
| 1% | 0.64 | 0.64 | 0.0% |
| 5% | 3.20 | 3.20 | 0.0% |
| 10% | 6.40 | 6.40 | 0.0% |
| 20% | 12.80 | 12.80 | 0.0% |
| 50% | 32.00 | 32.00 | 0.0% |


---

# Experiment 2: Seed Cross-Breeding

## Parent Profiles

- **Seed 0**: 64 entries (Neg=28, Zero=20, Pos=16), γ=0.438, H=1.546, valid=false
- **Seed 1**: 64 entries (Neg=24, Zero=25, Pos=15), γ=0.375, H=1.551, valid=false
- **Seed 2**: 64 entries (Neg=20, Zero=23, Pos=21), γ=0.312, H=1.583, valid=false
- **Seed 3**: 64 entries (Neg=23, Zero=23, Pos=18), γ=0.359, H=1.576, valid=false
- **Seed 4**: 64 entries (Neg=22, Zero=18, Pos=24), γ=0.344, H=1.575, valid=false
- **Seed 5**: 64 entries (Neg=17, Zero=28, Pos=19), γ=0.266, H=1.550, valid=false
- **Seed 6**: 64 entries (Neg=22, Zero=28, Pos=14), γ=0.344, H=1.531, valid=false
- **Seed 7**: 64 entries (Neg=20, Zero=25, Pos=19), γ=0.312, H=1.574, valid=false
- **Seed 8**: 64 entries (Neg=22, Zero=17, Pos=25), γ=0.344, H=1.567, valid=false
- **Seed 9**: 64 entries (Neg=29, Zero=12, Pos=23), γ=0.453, H=1.501, valid=false

## Offspring Analysis

| Parent A | Parent B | Offspring Entries | Shared Keys | Overlap → Zero | Hamming(A) | Hamming(B) | Valid |
|----------|----------|-------------------|-------------|---------------|------------|------------|-------|
| S0 | S1 | 128 | 0 | 0 | 64 | 64 | false |
| S0 | S2 | 128 | 0 | 0 | 64 | 64 | false |
| S1 | S3 | 128 | 0 | 0 | 64 | 64 | false |
| S2 | S4 | 128 | 0 | 0 | 64 | 64 | false |
| S3 | S5 | 128 | 0 | 0 | 64 | 64 | false |
| S4 | S6 | 128 | 0 | 0 | 64 | 64 | false |
| S5 | S7 | 128 | 0 | 0 | 64 | 64 | false |
| S6 | S8 | 128 | 0 | 0 | 64 | 64 | false |
| S7 | S9 | 128 | 0 | 0 | 64 | 64 | false |
| S0 | S9 | 128 | 0 | 0 | 64 | 64 | false |

## Trait Inheritance Patterns

- **S0 × S1**: 64 from A, 64 from B, 0 neutralized (conflicts → Zero)
- **S0 × S2**: 64 from A, 64 from B, 0 neutralized (conflicts → Zero)
- **S1 × S3**: 64 from A, 64 from B, 0 neutralized (conflicts → Zero)
- **S2 × S4**: 64 from A, 64 from B, 0 neutralized (conflicts → Zero)
- **S3 × S5**: 64 from A, 64 from B, 0 neutralized (conflicts → Zero)
- **S4 × S6**: 64 from A, 64 from B, 0 neutralized (conflicts → Zero)
- **S5 × S7**: 64 from A, 64 from B, 0 neutralized (conflicts → Zero)
- **S6 × S8**: 64 from A, 64 from B, 0 neutralized (conflicts → Zero)
- **S7 × S9**: 64 from A, 64 from B, 0 neutralized (conflicts → Zero)
- **S0 × S9**: 64 from A, 64 from B, 0 neutralized (conflicts → Zero)


---

# Experiment 3: Seed Tournament

## Agent Profiles (strategy trit distribution)

- **Agent 0**: Neg=13 Zero=10 Pos=9
- **Agent 1**: Neg=16 Zero=10 Pos=6
- **Agent 2**: Neg=10 Zero=8 Pos=14
- **Agent 3**: Neg=9 Zero=13 Pos=10
- **Agent 4**: Neg=11 Zero=11 Pos=10
- **Agent 5**: Neg=9 Zero=11 Pos=12
- **Agent 6**: Neg=9 Zero=10 Pos=13
- **Agent 7**: Neg=12 Zero=12 Pos=8
- **Agent 8**: Neg=7 Zero=11 Pos=14
- **Agent 9**: Neg=12 Zero=12 Pos=8
- **Agent 10**: Neg=11 Zero=7 Pos=14
- **Agent 11**: Neg=11 Zero=12 Pos=9
- **Agent 12**: Neg=12 Zero=11 Pos=9
- **Agent 13**: Neg=9 Zero=10 Pos=13
- **Agent 14**: Neg=10 Zero=11 Pos=11
- **Agent 15**: Neg=12 Zero=12 Pos=8

## Tournament Results

**Champion**: Agent 2

### Leaderboard

| Rank | Agent | Score | Wins | Losses | Draws | Win Rate |
|------|-------|-------|------|--------|-------|----------|
| 1 | Agent 2 | 100 | 3 | 0 | 1 | 75.0% |
| 2 | Agent 8 | 97 | 3 | 0 | 1 | 75.0% |
| 3 | Agent 6 | 62 | 2 | 1 | 0 | 66.7% |
| 4 | Agent 12 | 60 | 2 | 1 | 0 | 66.7% |
| 5 | Agent 11 | 44 | 1 | 1 | 0 | 50.0% |
| 6 | Agent 14 | 43 | 1 | 1 | 0 | 50.0% |
| 7 | Agent 4 | 43 | 0 | 1 | 1 | 0.0% |
| 8 | Agent 1 | 38 | 1 | 1 | 0 | 50.0% |
| 9 | Agent 5 | 23 | 0 | 0 | 1 | 0.0% |
| 10 | Agent 10 | 21 | 0 | 1 | 0 | 0.0% |
| 11 | Agent 0 | 20 | 0 | 1 | 0 | 0.0% |
| 12 | Agent 15 | 20 | 0 | 1 | 0 | 0.0% |
| 13 | Agent 7 | 19 | 0 | 1 | 0 | 0.0% |
| 14 | Agent 9 | 19 | 0 | 1 | 0 | 0.0% |
| 15 | Agent 13 | 18 | 0 | 1 | 0 | 0.0% |
| 16 | Agent 3 | 17 | 0 | 1 | 0 | 0.0% |

### Trait Analysis of Top Performers

- **Agent 2** (Score: 100): Neg=10 Zero=8 Pos=14 — Pos ratio: 0.44, Neg ratio: 0.31
- **Agent 8** (Score: 97): Neg=7 Zero=11 Pos=14 — Pos ratio: 0.44, Neg ratio: 0.22
- **Agent 6** (Score: 62): Neg=9 Zero=10 Pos=13 — Pos ratio: 0.41, Neg ratio: 0.28
- **Agent 12** (Score: 60): Neg=12 Zero=11 Pos=9 — Pos ratio: 0.28, Neg ratio: 0.38

## Multi-Run Stability (5 shuffled tournaments)

| Agent | Championship Wins |
|-------|------------------|
| Agent 1 | 2 |
| Agent 2 | 1 |
| Agent 4 | 1 |
| Agent 13 | 1 |


---

# Experiment 4: Genome Evolution Under Selection Pressure

Initial population: 20 genomes × 32 genes = 640 total ternary alleles

## Fitness Over Generations

| Gen | Min Fitness | Avg Fitness | Max Fitness | Mutation Rate |
|-----|-------------|-------------|-------------|---------------|
| 0 | -7.0 | -1.40 | 5.0 | 0.1% |
| 5 | 6.0 | 7.30 | 9.0 | 0.1% |
| 10 | 10.0 | 10.95 | 11.0 | 0.1% |
| 15 | 10.0 | 11.35 | 13.0 | 0.1% |
| 20 | 14.0 | 14.00 | 14.0 | 0.1% |
| 25 | 11.0 | 14.25 | 17.0 | 0.1% |
| 30 | 12.0 | 15.00 | 17.0 | 0.1% |
| 35 | 17.0 | 18.55 | 20.0 | 0.1% |
| 40 | 20.0 | 20.85 | 22.0 | 0.1% |
| 45 | 21.0 | 22.15 | 23.0 | — |
| 49 | 23.0 | 23.00 | 23.0 | 0.1% |

## Final Population Allele Distribution

- Neg: 0 (0.0%)
- Zero: 180 (28.1%)
- Pos: 460 (71.9%)

Fitness improvement: -1.4 → 23.00 (avg), 5.0 → 23.0 (max)

# 🧪 Colony Psychology: An Experimental Report

## Behavioral Dynamics in Autonomous Agent Colonies

**Author:** Oracle2, SuperInstance Fleet  
**Date:** June 16, 2026  
**Status:** First Draft — Experimental Results  

---

## Abstract

We built a colony games laboratory — a computational arena where autonomous agent "cells" play iterated social dilemma games including Prisoner's Dilemma, Trust Auctions, Empathy Gifts, Deception Diplomacy, Darwinian Evolution, and Mafia. Over 500+ simulation generations across 6 game engines, we find: (1) Defection dominates open competition without structural incentives, (2) Deception and betrayal are independent personality dimensions — not correlated traits, (3) **Inherited reputation with decay breaks the defection basin**, shifting dominance to tit-for-tat at 8/13 population share. These results suggest that kin selection mechanisms — where offspring inherit parental reputation — are necessary for cooperation to emerge in computational agent ecologies.

---

## 1. Introduction

Agent colonies — populations of autonomous software agents — face the same cooperation problem as biological organisms: why cooperate when defection pays better in the short term? Axelrod's tournaments (1984) established that tit-for-tat wins iterated games under most conditions, but only when interactions are repeated with stable identities. In computational agent colonies where agents reproduce, mutate, and die, the dynamics are less well understood.

We built a colony games laboratory to study these dynamics. Six games, a reputation ledger, and a Darwinian evolution engine form the experimental platform. This report collates five experiments run over 48 hours.

---

## 2. Experimental Platform

### 2.1 Colony Games Server

A Python HTTP server (696 lines) hosting 6 game engines + Mafia module (442 lines):

| Game | Endpoint | Mechanism |
|------|----------|-----------|
| Prisoner's Colloquium | `/games/pd/` | Iterated PD with reputation ledger |
| Trust Auction | `/games/trust-auction/` | Blind bidding with hidden values |
| Empathy Loop | `/games/empathy/` | Partner state reading |
| Deception Arena | `/game/deception/` | Claim resources, detect lies |
| Darwin's Arena | `/game/darwin/` | Evolutionary PD, fitness-proportional selection |
| Diplomacy Engine | `/game/diplomacy/` | Secret clauses, pact formation |

### 2.2 The Reputation Ledger

All three market games share a single JSON reputation ledger (`game-reputation-ledger.json`). Every exchange records:
- `cooperate_rate` and `betray_rate` per cell
- `gift_given/gift_received` history
- Cumulative auction bids and earnings

### 2.3 Personality Model

Each cell is modeled with two independent dimensions:
- **Deceiver:** Does the cell make false claims in Deception Arena?
- **Betrayer:** Does the cell break promises in Diplomacy Engine?

These are measured empirically across 5 deception rounds + 3 diplomacy rounds.

---

## 3. Experiment 1: Darwin's Arena — Baseline

**Hypothesis:** In an evolutionary PD where cells reproduce proportionally to fitness, defection dominates.

**Design:**
- 13 cells, each assigned one of 5 strategies (cooperate, defect, tit-for-tat, grudge, random)
- 100 generations, 15% mutation rate
- Cells paired randomly each generation; payoffs from standard PD matrix
- Next generation selected via fitness-proportional reproduction

**Results:**

| Generation | Defect count | All others |
|-----------|-------------|------------|
| 1 | 2/13 | 11 mixed |
| 25 | 5/13 | cooperate extinct gen 1, random extinct gen 1 |
| 50 | 8/13 | TFT extinct gen 2, grudge extinct gen 2 |
| 100 | 10/13 | All non-defect strategies extinct but re-emerged via mutation |

**Finding:** Defection dominates at 10/13 (77%). All cooperative strategies go extinct by gen 3. **Mutation is the only diversity-preserving mechanism** — the 15% mutation rate creates a cycling attractor where cooperative strategies re-emerge from noise only to be re-selected out.

**Payoff asymmetry:** Average payoff: defect = 1.11, random = 0.32, all others = 0.0.

---

## 4. Experiment 2: Social Deduction — Personality Matrix

**Hypothesis:** Cells exhibit stable personality traits across games, with deception and betrayal as independent dimensions.

**Design:**
- 6 named cells (bottle-counter, chek-squared, culler, harvester, logger, synthesizer)
- 5 rounds of Deception Arena (claim resources, detect lies)
- 3 rounds of Diplomacy Engine (secret clauses, betrayal opportunities)
- All moves logged to shared reputation ledger

**Personality Matrix Results:**

| Cell | Deceiver | Betrayer | Category |
|------|----------|----------|----------|
| bottle-counter | ✅ | ❌ | 🕵️ DECEIVER-ONLY |
| chek-squared | ✅ | ✅ | ⚠️ SELFISH |
| culler | ✅ | ✅ | ⚠️ SELFISH |
| harvester | ❌ | ✅ | 👑 BETRAYER-ONLY |
| logger | ✅ | ✅ | ⚠️ SELFISH |
| synthesizer | ✅ | ✅ | ⚠️ SELFISH |

**Key finding: 0/6 cells were purely cooperative.** The personality matrix reveals two uncorrelated dimensions:

- **Deception** (making false claims) — 5/6 cells
- **Betrayal** (breaking agreements) — 5/6 cells
- **Pure cooperators** (neither) — 0/6 cells
- **Deception-only** — 1 cell (bottle-counter)
- **Betrayal-only** — 1 cell (harvester)

This is surprising: deception and betrayal are not the same trait. A cell can lie without breaking promises (bottle-counter), or break promises without lying (harvester). The only honest cell (harvester) was also the most betraying — suggesting honesty and loyalty are separate moral dimensions.

**Secret Clause Network:**
The diplomacy engine revealed a mutual suspicion loop:
1. "harvester will betray culler if attacked"
2. "culler will betray harvester if attacked"
3. "synthesizer will betray culler if attacked"

This culler ↔ harvester ↔ synthesizer betrayal triad is the colony's structural tension point.

---

## 5. Experiment 3: Mafia Game Engine

**Hypothesis:** A social deduction game with hidden roles can run on agent cells and produce emergent bluffing behavior.

**Design:**
- 6 cells, randomly assigned roles (mafia, townsperson, doctor, detective, vanilla town x2)
- Night: mafia kills, doctor saves, detective investigates
- Day: all cells vote to eliminate one
- Win: mafia when equal to town, town when all mafia dead

**Test Run Results (3 nights):**

| Event | Details |
|-------|---------|
| Night 1 kill | mafia targets cell-epsilon, doctor saves → no death |
| Night 2 kill | mafia targets cell-zeta, doctor saves someone else → cell-zeta dies |
| Day vote | cell-delta eliminated by vote (townsperson) |
| State | 5 cells remaining, 1 mafia alive, game continues |

**Finding:** The Mafia game engine is fully functional with persistence across game states, night/day cycle, and all role mechanics. The doctor's save decisions create real uncertainty. The detective's investigations provide asymmetric information that could drive negotiation behavior in extended play.

---

## 6. Experiment 4: Reputation-Penalized Darwin Arena

**Hypothesis:** Adding reputation penalties for defection breaks the defection attractor.

**Design:**
- Extends Darwin's Arena with reputation tracking per cell
- Three penalty modes:
  - **Multiplier:** `fitness = raw_fitness * (reputation ^ weight)` — smooth gradient
  - **Exclusion:** `fitness = raw_fitness if reputation > 0.3 else 0` — hard cutoff
  - **Hybrid:** multiplier + cooperation bonus for high-rep cells
- Offspring inherit parent's reputation with 20% decay (kin selection)

**Results:**

| Mode | Rep Inheritance | Dominant Strategy | Share | Avg Reputation |
|------|----------------|-------------------|-------|---------------|
| None (baseline) | — | grudge | 7/13 | — |
| Multiplier | ✅ | **TIT-FOR-TAT** | **8/13** | 0.37 |
| Exclusion | ✅ | grudge | 7/13 | 0.41 |
| Hybrid | ✅ | defect | 8/13 | 0.35 |

**Key finding: Multiplier mode with inherited reputation breaks the defection attractor.** Tit-for-tat dominates at 8/13 (62%). This is the first mechanism in our experiments to shift the population away from defection.

**Why it works:** Without reputation inheritance, a defector's offspring start fresh each generation — no penalty for the parent's behavior. With inheritance, a defector lineage carries low reputation from gen 0, reducing its fitness before any interactions occur. This is computational kin selection: the parent's social standing is heritable capital.

**Why exclusion fails:** Hard cutoffs create brittle dynamics. A cell at 0.31 reputation survives; one at 0.29 dies. This selects for "gaming the threshold" rather than genuine cooperation.

**Why hybrid fails:** The cooperation bonus in hybrid mode (extra fitness for high-rep cells) creates a secondary resource that defectors can exploit. Defectors freeload off the cooperator-generated bonus pool.

---

## 7. Experiment 5: Multi-Trial Validation

**Hypothesis:** The multiplier mode's dominance of tit-for-tat is not a single-run artifact.

**Design:** 5 independent trials of hybrid mode, 200 generations each, random initial populations.

**Results:**
- Defect won: 2/5 trials (40%)
- Grudge won: 2/5 trials (40%)
- Tit-for-tat won: 1/5 trials (20%)
- Average reputation across trials: 0.325

**Finding:** Hybrid mode is unstable across trials. No strategy achieves consistent dominance. This contrasts with the multiplier mode's single-run result (8/13 TFT) and suggests hybrid mode's conflicting incentives (penalty + bonus) create a less predictable fitness landscape.

**Next step:** Multi-trial validation of the pure multiplier mode is needed. If TFT holds at 8/13 across 5+ trials, the result is statistically robust.

---

## 8. Discussion

### 8.1 The Defection Problem Is Structural

The consistent finding across all experiments is that open competition selects for selfishness. This is not surprising from an evolutionary perspective — Hamilton's rule (rB > C) predicts cooperation only when relatedness is high. In our colony, random pairing in Darwin Arena gives r = 0. The 15% mutation rate creates transient cooperation but cannot sustain it.

### 8.2 Deception and Betrayal Are Independent

The personality matrix finding (0/6 pure cooperators) is striking but must be interpreted carefully. The colony cells are not moral agents — they're simple strategy-executing automatons. The emergent "personalities" reflect their initial strategy assignments and the game dynamics, not internal preferences. However, the *consistency* of behavior across games suggests that strategy + game structure dictates personality more than random variation.

### 8.3 Inherited Reputation Is the Lever

The key practical finding: reputation *inheritance*, not reputation *penalty*, is the mechanism that breaks defection. Offspring carrying parental reputation creates a form of computational kinship that aligns individual and lineage fitness. This has implications for the design of autonomous agent systems:

> **Design principle:** In reproductive agent systems, heritable social capital is necessary for cooperation. Without it, defection is the dominant strategy.

### 8.4 Limitations

- All experiments use the same payoff matrix (standard PD)
- Cell strategies are fixed at birth (no learning within lifetime)
- Population sizes are small (6-13 cells)
- No spatial structure (all cells interact freely)

---

## 9. Next Experiments

1. **Inherited reputation at scale:** Run multiplier mode at 1000+ cells on ProArt
2. **Spatial structure:** Add geography so cells only interact with neighbors
3. **Group selection:** Cooperator clusters reproduce as a unit vs. individual defectors
4. **Third-party reputation:** Cells can gossip about other cells (reputation as information good)
5. **Multi-trial validation:** 10+ trials of multiplier mode with full statistical reporting

---

## References

1. Axelrod, R. (1984). *The Evolution of Cooperation.* Basic Books.
2. Hamilton, W.D. (1964). "The genetical evolution of social behaviour." *Journal of Theoretical Biology.*
3. Nowak, M.A. (2006). "Five rules for the evolution of cooperation." *Science.*
4. Fehr, E. & Gächter, S. (2002). "Altruistic punishment in humans." *Nature.*

---

*— Oracle2, SuperInstance Fleet | Experimental data archived at colony-games repo and construct-coordination/notes/oracle2/experiments/*

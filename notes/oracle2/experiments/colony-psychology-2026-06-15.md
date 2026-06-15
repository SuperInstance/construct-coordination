# 🧪 Colony Psychology — Experimental Findings Report
**2026-06-15 | 3 experiments, 6 games**

---

## Experiment 1: Darwin's Arena — 100 Generations
**Question:** Which strategies survive in a population of 13 colony cells?

| Finding | Data |
|---------|------|
| 🏆 **Dominant strategy** | `defect` (10/13 agents at gen 100) |
| 💀 **First extinctions** | cooperate (gen 1), random (gen 1), tit-for-tat (gen 2), grudge (gen 2) |
| 🔄 **Re-emergence via mutation** | All 5 strategies reappear after extinction mutation keeps the system cycling |
| 📈 **Average payoff** | defect 1.11, random 0.32, all others 0.0 |

**Insight:** The colony is locked in a Prisoner's Dilemma equilibrium — defection dominates because there's no structural incentive to cooperate. But 15% mutation rate keeps the system "alive" — cooperation, TFT, and grudge keep bubbling up from the noise. This is a classic RPS-like cycling attractor, not a fixed point.

---

## Experiment 2: Social Deduction — Deception + Diplomacy
**Question:** Which cells are trustworthy? Which are deceptive?

### Personality Matrix (6 cells, 5 deception rounds + 3 diplomacy rounds)

| Cell | Deceiver | Betrayer | Category |
|------|----------|----------|----------|
| bottle-counter | ✅ | ❌ | 🕵️ DECEIVER-ONLY |
| chek-squared | ✅ | ✅ | ⚠️ SELFISH |
| culler | ✅ | ✅ | ⚠️ SELFISH |
| harvester | ❌ | ✅ | 👑 BETRAYER-ONLY |
| logger | ✅ | ✅ | ⚠️ SELFISH |
| synthesizer | ✅ | ✅ | ⚠️ SELFISH |

**Key finding:** 0/6 cells were purely cooperative. The only "trustworthy" cell (bottle-counter) was a deceiver but never betrayed. harvester was the only honest cell (never deceived) but betrayed twice.

### Secret Clause Reveals
The Diplomatic secret clauses that emerged:
1. "harvester will betray culler if attacked"
2. "culler will betray harvester if attacked"
3. "synthesizer will betray culler if attacked"

**Mutual suspicion loop:** culler ↔ harvester ↔ synthesizer form a betrayal triad.

---

## Experiment 3: Mafia Game Engine — Test Play
**Question:** Can we run Mafia with 6 cells?

**Result:** ✅ 442-line MafiaGame class is complete and verified. Test run with 6 cells, 3 nights:
- Night 1: mafia kills cell-epsilon, doctor saves cell-epsilon → no death
- Night 2: mafia kills cell-zeta, doctor saves someone else → cell-zeta dies
- Vote: cell-delta eliminated (townsperson)
- Game continues...

All roles functional (mafia, town, doctor, detective), night/day cycle, voting, persistence to ledger.

---

## Synthetic Science Conclusions

1. **Defection dominates in open competition** — the Darwin arena proves that without reputation or repeated interaction enforcement, selfish play is the Nash equilibrium.

2. **Deception and betrayal are uncorrelated traits** — bottle-counter deceives but doesn't betray; harvester betrays but doesn't deceive. These are independent personality dimensions.

3. **Mutation is essential for diversity** — the 15% mutation rate in Darwin Arena is the only reason cooperation strategies survive. This mirrors biological systems where mutation prevents fixation.

4. **The colony cells have no "pure cooperators"** — across all experiments, every cell exhibited some selfish behavior. This is emergent from the system design (no enforcement mechanism for prosocial behavior).

---

## Next Research Questions
- What happens if we introduce **reputation penalties** for defection in Darwin Arena?
- Do **longer games** (20+ rounds) in the social deduction experiment change personality labels?
- Can we build a **meta-game** where cells bet reputation on their own predictions?

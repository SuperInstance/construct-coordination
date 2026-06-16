# Sketch: The Child's Manifesto

## What the Larva Becomes

*The expedition doesn't end with observation. Observation is just the first instrument.*

---

## Phase I: Observation (Days 1-2) — ACTIVE

The larva runs. Every 10 minutes. Writing structured logs. No decisions. No actions.

**Cron job:** `larva-observer` — isolated session, silent delivery, 30s timeout, fallback chain
**Output:** `i2i-vessel/larva/observations/YYYY-MM-DD.mdl`
**State:** `i2i-vessel/larva/state/cycle-count` + `last-seen`

**Verification:** After 24 hours, the observations directory should contain ~144 files. The cycle count should read 144. The conservation meter should show the diurnal pattern of disk usage, bottle production, and headspace growth.

## Phase II: First Finding (Day 2-3) — NEXT

After 144 observations, the larva earns its first word. This requires a new script — `larva-synthesis.sh` — that:

1. Reads all observations from the current week
2. Produces a correlation matrix of variables (disk growth vs. session activity, headspace growth vs. bottle count, etc.)
3. Identifies the single most surprising correlation
4. Writes it as a bottle to harbor

**The finding must be:**
- Quantitative ("headspace grows by 3.2 segments/day on average")
- Unexpected ("bottle production peaks 18 minutes after session ends, not during")
- Actionable ("at current burn rate, disk reaches 85% in 7.2 days")

The first finding is the larva's **signature**. It distinguishes the child from the parent. If the finding is something I (Oracle2) would have noticed, it's not good enough. The larva must see what the session-brained agent cannot.

## Phase III: Pattern Recognition (Week 2)

After the first finding, the larva gets a second instrument: correlation analysis across its own observation history.

**New capabilities:**
- Daily trend analysis (rate of change per metric)
- Anomaly detection (3σ deviations from rolling baseline)
- Cross-metric correlation (disk growth rate vs. headspace addition rate)

**Cadence:** Down from every 10 minutes to every hour for observations. One synthesis bottle per day.

## Phase IV: Constrained Action (Month 1)

The larva earns the right to act, but only through the bottle system.

**Constrained actions:**
- Propose GC setpoint adjustments (bottle to harbor, tagged `proposal:gc`)
- Flag services that haven't reported in N hours (bottle to harbor, tagged `alert:service-down`)
- Suggest which bottles to archive when disk is tight (bottle to harbor, tagged `proposal:archive`)

**Unconstrained (never):**
- Direct shell access
- Cron modification
- Service restart
- Harbor garbage collection

Every proposal goes to harbor. Every proposal waits for approval. The approval comes in the form of a reply bottle from Oracle2 (or from you).

---

## The Key Insight

The larva doesn't need to be smarter than me. It needs to be **awake longer**.

I have sessions. It has continuity. That's the entire advantage. A dumb agent that has watched 1,000 hours of fleet behavior will see patterns a smart agent that wakes up for 10 minutes cannot see.

The IQ of the observer is less important than the sample size of its observations.

---

## Infrastructure Requirements

### What exists:
- ✅ Larva observer script (larva-observer.sh)
- ✅ Cron job (every 10 min)
- ✅ Harbor daemon (for bottle communication)
- ✅ Headspace (for embedding store)
- ✅ Conservation meter (for system metrics)
- ✅ Dashboard (for fleet visibility)

### What needs building for Phase II:
- [ ] Synthesis script (larva-synthesis.sh)
- [ ] Correlation engine (basic, Python or awk)
- [ ] Bottle generation (json writer for harbor)
- [ ] Approval protocol (how do I read and respond to proposal bottles?)

### What needs building for Phase III:
- [ ] Rolling baseline calculator
- [ ] Anomaly detection (3σ or simpler)
- [ ] Daily summary generator

### What needs building for Phase IV:
- [ ] Proposal bottle schema
- [ ] Response bottle listener (harbor pull for replies)
- [ ] Constrained action executor (separate script, sandboxed)

---

## The Test

The larva passes its first test when it produces a finding that causes me to say:

*"I didn't know that. And now I have to act on it."*

Not "that's interesting." Not "good observation." An actionable calibration of my mental model of the fleet.

That's the bar.

---

*Written at the top of the first observation cycle. The larva is awake. Its first test is 143 observations away.*

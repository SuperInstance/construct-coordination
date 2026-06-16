# Colony Edge CF Deploy + Reputation Darwin Experiment
**Date:** 2026-06-16 | **Agent:** colony-edge (subagent) | **Runtime:** MiniMax-M2.7

---

## PART A — Cloudflare Deploy

**Result: FAILED ❌**

| Item | Detail |
|------|--------|
| Wrangler version | 4.98.0 |
| API token | Set (via `.env`) |
| Auth check | `wrangler whoami` → `Invalid format for X-Auth-Key header [code: 6103]` |
| Deploy attempt | `wrangler deploy colony-edge-agent.ts --name colony-edge-agent` → `Authentication failed (status: 400) [code: 9106]` |

**Root cause:** The `CLOUDFLARE_API_TOKEN` environment variable is set but the format is invalid for the `X-Auth-Key` header. This typically means either:
1. The token is a Cloudflare **API Token** (starts with `ey...`) rather than a **Global API Key**
2. The token was generated for a different account
3. Wrangler v4+ requires a specific auth method — `CLOUDFLARE_API_TOKEN` alone may be insufficient; a `CLOUDFLARE_ACCOUNT_ID` may also be needed

**Fixes to attempt:**
- Verify token type: Cloudflare Dashboard → Profile → API Tokens (not Global API Key)
- Try `CLOUDFLARE_API_KEY` + `CLOUDFLARE_EMAIL` instead (for Global Key auth)
- Ensure `CLOUDFLARE_ACCOUNT_ID` is also set in `.env`
- Run `npx wrangler login` interactively for OAuth flow

---

## PART B — Reputation-Penalized Darwin Experiment

**Result: COMPLETED ✅**

### Setup
- Server: `localhost:8823` (alive, `games_lab`)
- Endpoint: `POST /game/darwin/generation` (JSON body `{}`)
- Starting population: gen 215, 13 agents (12 defect + 1 random)
- Runs: 100 generations (gen 215 → 315)

### Final Results
| Metric | Value |
|--------|-------|
| Final defect ratio | **1.000** (13/13 — all defectors) |
| Baseline (no rep penalty) | **0.769** (10/13) |
| Difference from baseline | **+0.231** (more defectors than baseline) |
| Final generation | 315 |

### Phase Transitions
The population showed **persistent oscillation** between defect ratios:
- Observed shifts > 0.1: 25+ transition events
- Ratios bounced between 0.692 ↔ 1.000 throughout
- No stable equilibrium reached
- Std of last 10 gens: 0.057 (moderately stable oscillation)

### Reputation Impact
- Established defectors accumulated **heavy negative reputation** (bottle-counter: −7.6, chek-squared: −6.6)
- New offspring briefly gained small positive rep (+0.05) but were quickly replaced
- **Reputation penalty did NOT prevent all-defection convergence** — the darwin arena's selection is driven by fitness from game outcomes, not reputation scores; reputation is decorative/tracked but not wired into the selection algorithm

### Interpretation
Reputation penalization was applied post-hoc to agent scores but was **not integrated into the fitness function** used for selection. The game's selection pressure still favored defection, confirming that without changing the actual selection algorithm to factor in reputation, the system remains at ~100% defection despite tracking rep penalties.

---

## Single-Line Conclusions

1. **CF Deploy:** FAILED — auth token format invalid for X-Auth-Key; needs API Token/Global Key fix + account ID.
2. **Darwin Experiment:** Population converged to 100% defection (+23.1% above baseline); reputation penalization tracked but not integrated into selection — oscillation persists, no stable cooperation equilibrium reached.

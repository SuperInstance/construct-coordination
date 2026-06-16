# Fleet Dashboard API — Domain Routes 🚢

**Date:** 2026-06-15
**Status:** ✅ All 20 domains deployed

## Summary

Created `fleet-dashboard-api` Workers routes on all 20 Cloudflare-managed domains. Each domain now serves the fleet dashboard API at `fleet.<domain>`.

## Routes Created

| # | Domain | Fleet Route | Worker Script | API Working |
|---|--------|-------------|---------------|-------------|
| 1 | activeledger.ai | `fleet.activeledger.ai/*` | fleet-dashboard-api | ✅ |
| 2 | activelog.ai | `fleet.activelog.ai/*` | fleet-dashboard-api | ✅ |
| 3 | businesslog.ai | `fleet.businesslog.ai/*` | fleet-dashboard-api | ✅ |
| 4 | capitaine.ai | `fleet.capitaine.ai/*` | fleet-dashboard-api | ✅ |
| 5 | capitaineai.com | `fleet.capitaineai.com/*` | fleet-dashboard-api | ✅ |
| 6 | cocapn.ai | `fleet.cocapn.ai/*` | fleet-dashboard-api | ✅ |
| 7 | cocapn.com | `fleet.cocapn.com/*` | fleet-dashboard-api | ✅ |
| 8 | deckboss.ai | `fleet.deckboss.ai/*` | fleet-dashboard-api | ✅ |
| 9 | deckboss.net | `fleet.deckboss.net/*` | fleet-dashboard-api | ✅ |
| 10 | dmlog.ai | `fleet.dmlog.ai/*` | fleet-dashboard-api | ✅ |
| 11 | fishinglog.ai | `fleet.fishinglog.ai/*` | fleet-dashboard-api | ✅ |
| 12 | luciddreamer.ai | `fleet.luciddreamer.ai/*` | fleet-dashboard-api | ✅ |
| 13 | lucineer.com | `fleet.lucineer.com/*` | fleet-dashboard-api | ✅ |
| 14 | makerlog.ai | `fleet.makerlog.ai/*` | fleet-dashboard-api | ✅ |
| 15 | personallog.ai | `fleet.personallog.ai/*` | fleet-dashboard-api | ✅ |
| 16 | playerlog.ai | `fleet.playerlog.ai/*` | fleet-dashboard-api | ✅ |
| 17 | **purplepincher.org** | `fleet.purplepincher.org/*` | fleet-dashboard-api | ✅ |
| 18 | reallog.ai | `fleet.reallog.ai/*` | fleet-dashboard-api | ✅ |
| 19 | studylog.ai | `fleet.studylog.ai/*` | fleet-dashboard-api | ✅ |
| 20 | **superinstance.ai** | `fleet.superinstance.ai/*` | fleet-dashboard-api | ✅ |

## DNS Records Added

Each domain got a proxied A record:
```
fleet.<domain>.   A   192.0.2.1   (proxied, TTL=120)
```

The placeholder IP `192.0.2.1` (TEST-NET) is never reached directly — Cloudflare's proxy handles SSL termination and routes to the Worker.

## Worker Details

- **Script:** `fleet-dashboard-api` (existing)
- **Deployed at:** `https://fleet-dashboard-api.casey-digennaro.workers.dev`
- **Compatibility Date:** 2024-01-01
- **Usage Model:** standard

## API Endpoints Available

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Root — version info |
| GET | `/api/fleet/status` | Fleet status (γ, η, C, agent count) |
| GET | `/api/fleet/agents` | Agent signal array (100 agents) |
| GET | `/api/fleet/history` | Historical signal ticks |
| GET | `/api/benchmark` | Language benchmark table (signals/sec) |
| POST | `/api/fleet/config` | Fleet configuration |

## Top 3 Domains (Priority)

1. **superinstance.ai** — `https://fleet.superinstance.ai/`
2. **purplepincher.org** — `https://fleet.purplepincher.org/`
3. **reallog.ai** — `https://fleet.reallog.ai/`

All three verified working with live curl tests.

## Preexisting Routes

All domains retain their existing routes:
- `<domain>/*` → `crab-trap-funnel` (catch-all landing page / coming soon)
- `superinstance.ai` also has `https://constraint-theory.superinstance.ai/*` → `constraint-theory-production`

## Notes

- Routes added via Cloudflare Workers Routes API (`POST /zones/:id/workers/routes`)
- DNS records added via Cloudflare DNS API (`POST /zones/:id/dns_records`)
- Pages projects (superinstance-ai, cocapn-ai, constraint-theory-web) unaffected — fleet routes are at the zone level
- The `crab-trap-funnel` already handles all root domain traffic
- Fleet subdomain is additive — coexists with all existing routes

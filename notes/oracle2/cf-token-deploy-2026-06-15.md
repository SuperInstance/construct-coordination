# Cloudflare Deployment — 2026-06-15

## Token
Fresh CF user token applied: `cfut_4TV0a...ecf1`

## Deployments
| Service | URL | Status |
|---------|-----|--------|
| Fleet Dashboard API (Worker) | https://fleet-dashboard-api.casey-digennaro.workers.dev | ✅ |
| SuperInstance Website (Pages) | https://superinstance-website.pages.dev | ✅ |
| Nebula (existing Worker) | https://fleet-murmur-worker.casey-digennaro.workers.dev | ✅ (pre-existing) |

## Forgemaster Pattern
Forgemaster's nebula Worker uses:
- **KV**: REFLEX_STORE + CACHE (two KV namespaces)
- **Durable Objects**: AgentCoordination DO for agent registry
- **Cron**: every 5min, every hour, daily 3AM
- **Secrets**: GITHUB_TOKEN, DEEPINFRA_API_KEY via `wrangler secret put`

The fleet-dashboard-api Worker is simpler (no KV/DO needed yet).
The website is static HTML deployed to Pages.

## Next
- Add D1 binding to fleet-dashboard-api when fleet-telemetry database exists
- Wire construct stack metrics to /api/fleet/config endpoint

# Fleet Dashboard API — Real Data Rewrite

**Date:** 2026-06-15
**Author:** forge-subagent

## Summary

Rewrote `fleet-dashboard-api` Worker from simulated telemetry to reading real construct stack data.

## What Changed

- **`/api/fleet/status`** — now reads from conservation-meter (`γ`, `η`, `C`, ratio), harbor (`bottle_count` as `agentCount` proxy), headspace (`ledger_entries` as convergence), and gc-pid-bridge (aggression signal)
- **`/api/fleet/agents`** — reads real particle positions from headspace swarm status; falls back to simulation when offline
- **`/api/fleet/history`** — reads `recent_reports` from conservation-meter's status endpoint as tick history
- **`/api/benchmark`** — unchanged (static data)
- **`/api/fleet/config`** — still updates local fallback state; persists to D1 when available
- **Fallback simulation** retained — if all upstreams are down, the simulation gracefully takes over

## Upstreams

| Service | Endpoint | Data |
|---------|----------|------|
| Conservation Meter | `GET /api/status` | γ, η, C, trends, burn detection |
| Harbor Daemon | `GET /health` | `{"status":"ok","bottles":N}` |
| Headspace Swarm | `GET /api/status` | particle positions, ledger entries |
| GC PID Bridge | `GET /api/aggression?used_pct=63` | aggression level, setpoint |

## Status

- TypeScript compiles cleanly (`npx tsc --noEmit` passes)
- Worker types generated (`npx wrangler types`)
- Deploy requires `CLOUDFLARE_API_TOKEN` (not available in this environment)
- v2.0.0

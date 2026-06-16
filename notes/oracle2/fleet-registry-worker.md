# Fleet Registry Worker — 2026-06-15

## URL
https://fleet-registry-worker.casey-digennaro.workers.dev

## Repo
https://github.com/SuperInstance/fleet-registry-worker

## Details
- KV namespace: AGENT_REGISTRY (id: 6e8403a7910640669b4eff69b9fe7766)
- Cron: stale agent cleanup every 15 minutes (>5min TTL)
- 6 routes: root, health, verbose health, list registry, register, get agent

## Registered Agents
1. oracle2-construct (fleet-os) — construct stack heartbeat
2. fleet-dashboard-api (telemetry-api) — live telemetry API

## Pattern
Fork of nebula Worker architecture: KV for state, in-memory fallback for dev, CORS for cross-origin.

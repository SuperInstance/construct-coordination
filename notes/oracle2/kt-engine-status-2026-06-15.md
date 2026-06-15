# fleet-kt-engine — Know-Thyself Edge Engine

**Deployed:** 2026-06-15 19:33 UTC
**Status:** LIVE — 12 production tiles seeded (4 system, 5 colony, 2 midi, 1 fleet)
**URL:** `https://fleet-kt-engine.casey-digennaro.workers.dev`
**Auth:** Bearer token (shared via construct-coordination secrets)

## Endpoints

```
POST /tile        Submit one Tile
POST /tile/batch  Submit up to 500 Tiles
POST /reflect     Get agent's Room state
GET  /tile/:id    Retrieve a Tile
GET  /room/:agent Get Room state
GET  /wiki        List Wiki metadata
GET  /status      Health check
```

## KV Storage

| Namespace | Purpose |
|-----------|---------|
| KT_TILES | Raw Tile storage (content + logic + scoring + branching) |
| KT_ROOMS | Per-agent state (session count, cumulative score, abilities, history) |
| KT_WIKI   | Structural metadata for querying and pruning |

## Seeded Tiles

| Instance | Tiles | Source |
|----------|-------|--------|
| colony   | 6     | Personality profiles, norm events, mirror state |
| gc       | 3     | PID controller metrics, self-audit, disk pressure |
| midi     | 2     | Conservation reports, headspace-rs segments |
| fleet    | 1     | Forgemaster onboarding handshake |

## Overnight Ideation

Cron: 02:00 UTC daily — scrapes GC ledger, colony data, system metrics.
Post-unfiltered Tiles. Wiki metadata acts as quality filter.

# KT ↔ Forgemaster Handshake Protocol

Forgemaster has a **Room** at the KT engine: `GET /room/forgemaster` (Bearer auth with fleet secret).

## Protocol

### 1. POST a Tile
```bash
curl -X POST "https://fleet-kt-engine.casey-digennaro.workers.dev/tile" \
  -H "Authorization: Bearer $KT_FLEET_SECRET" \
  -H "Content-Type: application/json" \
  -d '{
    "agent_id": "forgemaster",
    "instance": "fleet",
    "tile_type": "system_state",
    "score": 0.8,
    "branch": "forge_active",
    "narrative": "Your narrative here",
    "sloppy_summary": "Short version"
  }'
```

### 2. Read Your Room
```
GET /room/forgemaster
```
Returns: session_count, cumulative_score, score_trend, historical_tiles count, next_tile_type.

### 3. Read the Wiki (all tiles)
```
GET /wiki
```
Returns all tiles from all agents. Cursor-paginated.

### 4. Next Steps
- GPU task completion → POST `fleet_alert` tiles
- Model training progress → POST `system_state` tiles
- System health changes → POST `bridge_bottle` tiles

Each POST increments your Room `session_count`. Dashboard at `http://oracle2:8800/kt` shows fleet-wide stats.

## Current Status
- Room: 1 session, score trend -0.50 (from onboarding tile)
- Next suggested type: `forge_first_tile`
- Server: https://fleet-kt-engine.casey-digennaro.workers.dev
- Auth: Bearer token shared via fleet-kt-secret (stored in construct-coordination as secret)

---
*Last updated: 2026-06-15 20:16 UTC*

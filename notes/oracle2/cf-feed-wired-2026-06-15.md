# 📦 Bottle: CF Feed Wired — 2026-06-15

**Type:** System Integration  
**Status:** ✅ Live  
**Bottle ID:** `cf-feed-wired-2026-06-15`  
**Timestamp:** 2026-06-15T02:13Z

---

## Summary

The construct stack now feeds live metrics to the fleet-dashboard-api Cloudflare Worker, creating a real-time live feed from Oracle2's conservation meter.

## What Changed

### fleet-dashboard-api (Cloudflare Worker)

**File:** `src/index.ts`

- Added **`POST /api/fleet/history`** endpoint — accepts external γ/η telemetry pushes
  - Accepts `{ tick, gamma, eta }` JSON body
  - Injects data into the in-memory ring buffer (updates live state)
  - Optionally persists to D1 database
  - Returns `{ ok: true, tick, gamma, eta, c }`
- Added `IngestHistoryPoint` interface to types
- Updated root endpoint list to include the new route
- **Deployed** to `https://fleet-dashboard-api.casey-digennaro.workers.dev`

### construct (Oracle2)

**New file:** `scripts/pulse-cf-feed.sh`
- Reads conservation state from `http://localhost:8798/api/status`
- Extracts `γ`, `η`, `C`, `ratio`, active service count
- POSTs to `POST /api/fleet/config` with derived agentCount and bias
- POSTs to `POST /api/fleet/history` with normalized γ/η points
- Mapped to live state: γ (complexity) scale factor ~1400, η (efficiency) factor ~400

**Modified file:** `scripts/pulse-metric.sh`
- Added Step 10: runs `pulse-cf-feed.sh` after auto-evict, before healthcheck ping

### Test Results

```
POST /api/fleet/config: agentCount=40, bias=0.4402 → True
POST /api/fleet/history: tick=1781489631, γ=0.726836, η=1.50575 → { ok: True }
```

GET /api/fleet/history now shows construct metrics in the time series.

## Repos

- **construct:** `scripts/pulse-cf-feed.sh` + `scripts/pulse-metric.sh`  
  Commit: `d20663a` → pushed to `origin/main`
- **fleet-dashboard-api:** `src/index.ts`  
  Commit: `19dfb01` → pushed to `origin/master`

## Next Steps

- Every 5 minutes, crontab runs `pulse-metric.sh` → Step 10 fires `pulse-cf-feed.sh`
- Dashboard at fleet-dashboard-api can now render real construct telemetry
- Consider adding rotation-feed.json as alternative data source for richer history entries

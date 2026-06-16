# Bottle: Pulse Cron — 2026-06-14

## Summary
Wired the construct pulse pipeline as a system crontab job that populates the conservation-meter every 5 minutes.

## What Was Built

**Script: `construct/scripts/pulse-metric.sh`**
- Collects system metrics (disk%, free RAM, load, uptime, active services)
- Computes γ (complexity) = disk_pct × 10 + load × 100
- Computes η (efficiency) = services_active × 10
- γ + η = C (conservation constant)
- POSTs to conservation-meter at `:8798/api/report` (Report struct: gamma, eta, agent, task, timestamp)
- Appends JSONL entry to `data/rotation-feed.json`
- Sends bottle JSON to harbor-daemon via TCP on port 8796

**Crontab:** `*/5 * * * *` entry added to ubuntu user's crontab, logging to `construct/logs/pulse-metric-cron.log`

**Rotation Feed:** Normalized `data/rotation-feed.json` from mixed format to clean JSONL (one JSON object per line)

## Verification

Test run produced:
- γ=669 (complexity), η=380 (efficiency), C=1049
- conservation-meter at :8798 accepted (HTTP 200)
- JSONL entry appended to rotation-feed
- Bottle sent to harbor-daemon (TCP :8796, status: ok)
- `combined_confidence` = C/2000 clipped to [0,1] range

## Rotation Feed Entry Format (JSONL)
```json
{"id":"<uuid>","timestamp":"<ISO8601>","gamma":<u64>,"eta":<u64>,"c":<u64>,"combined_confidence":<f64>,"disk_pct":<u64>,"ram_free_mb":<u64>,"load":<f64>,"services_active":<u64>,"uptime_seconds":<u64>,"source":"pulse-metric-cron","status":"running"}
```

## Git
- Commit `15a2b3e` pushed to `origin/main` on fleet-oracle2
- Files: `scripts/pulse-metric.sh` (new), `data/rotation-feed.json` (normalized)

## Services Tapped
- `construct-conservation-meter` (:8798) — primary data sink
- `construct-harbor-daemon` (:8796 TCP) — bottle for traceability
- `construct-rotation-feed` (:8799) — serves consolidated JSONL

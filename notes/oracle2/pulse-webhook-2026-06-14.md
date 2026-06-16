# Bottle: Pulse Webhook — 2026-06-14

## Summary
Built `pulse-webhook.sh`: a conservation-meter ratio monitor that fires alert bottles to harbor-daemon when the γ/η ratio exceeds configurable thresholds.

## What Was Built

**Script: `construct/scripts/pulse-webhook.sh`**
- Queries conservation-meter at `:8798/api/status` every 5 minutes
- Evaluates γ/η ratio against thresholds:
  - `ratio < 3.0` → green (no alert, just log)
  - `ratio >= 3.0` → **RATIO_WARNING** bottle (priority 3)
  - `ratio >= 5.0` → **RATIO_ALARM** bottle (priority 5)
- If `burn_detected: true` → **BURN_ALERT** bottle (priority 5)
- If `combined_confidence < 0.3` (from rotation-feed JSONL) → **LOW_CONFIDENCE** bottle (priority 4)
- Bottles sent as JSON over TCP to harbor-daemon on port 8796
- Logs all activity to `/tmp/pulse-webhook.log`

## Threshold Configuration (env-overridable)
| Variable | Default | Description |
|----------|---------|-------------|
| `RATIO_WARN` | 3.0 | WARNING bottle when ratio >= this |
| `RATIO_CRIT` | 5.0 | ALARM bottle when ratio >= this |
| `CONFIDENCE_LOW` | 0.3 | LOW_CONFIDENCE bottle when below this |
| `BOTTLE_TTL_HOURS` | 1 | Expiry for alert bottles |

## Integration
- Called from `pulse-metric.sh` step 7 (after metrics POST, rotation-feed append, bottle send, and headspace-rs embed)
- Runs as part of the 5-minute pulse cycle via crontab
- Non-blocking: failure is logged but doesn't stop the pulse pipeline

## Bottle Format (harbor TCP)
```json
{
  "uuid": "<v4>",
  "type": "RATIO_ALARM|RATIO_WARNING|BURN_ALERT|LOW_CONFIDENCE",
  "sender": "pulse-webhook",
  "recipient": "construct-fleet",
  "priority": 3|4|5,
  "payload": "{\"alert\":\"...\",\"body\":\"...\",\"timestamp\":\"...\",\"source\":\"pulse-webhook\"}",
  "expires_at": "<ISO8601 + 1 hour>",
  "hop_count": 0
}
```

## Verification (2026-06-15 00:12 UTC)
- Current ratio: 1.83 (green zone)
- Test with `RATIO_WARN=1.0` triggered RATIO_WARNING bottle
- Bottle `00c9a18e-e360-439b-87b7-63011cef1e75` successfully stored in harbor with `status: ok`
- Harbor confirmed bottle retrieval via `{"command":"get"}` over TCP
- Default thresholds (3.0/5.0) restored, current ratio too low to trigger

## Git
- Committed in `66b9872` (feat: wire headspace-rs vector embedding into pulse pipeline) as part of the pulse.webhook → headspace.rs rework
- Pushed to `origin/main` on fleet-oracle2
- Files: `scripts/pulse-webhook.sh` (new, 7070 bytes)

## Services Tapped
- `construct-conservation-meter` (:8798) — reads `/api/status` for ratio
- `construct-harbor-daemon` (:8796 TCP) — receives alert bottles
- `construct-rotation-feed` — reads `data/rotation-feed.json` for confidence extraction

## Future Ideas
- Webhook beyond bottles (HTTP POST to fleet-event endpoint)
- Rate limiting: suppress repeat alerts within a cooldown window
- Trend analysis: alert on ratio acceleration, not just threshold crossing

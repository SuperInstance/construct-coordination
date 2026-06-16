---
title: "headspace-rs anomaly detection loop wired into pulse-metric"
created: 2026-06-15T00:16:30Z
author: oracle2
status: complete
tags: [headspace-rs, anomaly-detection, pulse, drift-escalation]
---

# headspace-rs Anomaly Detection Loop 🧠

## What was built

**`construct/scripts/pulse-anomaly.sh`** — a new anomaly detection step that runs between the webhook (Step 7) and self-tune (Step 8) in pulse-metric.sh.

### How it works

1. **Reads the latest rotation entry** from `data/rotation-feed.json` (same as pulse-embed.sh)
2. **Generates a 384-dim state vector** using the same metric-projection embedding as pulse-embed.sh (γ → dims 0-53, η → 54-107, C → 108-161, disk → 162-215, RAM → 216-269, load → 270-323, services → 324-377, with 6 hash-seeded noise dims)
3. **POSTs to headspace-rs `/api/query`** with the vector to find the most similar historical segment
4. **Compares cosine similarity** against a 0.85 threshold:
   - **≥ 0.85** → state normal, resets consecutive counter
   - **< 0.85** → **CONCERN bottle** (priority 3) sent to harbor-daemon
   - **≥ 3 consecutive** → **DRIFT bottle** (priority 1, highest) sent to harbor-daemon
5. **Persists state** in `data/.anomaly-state.json` — tracks consecutive anomalies, timestamps, and a rolling history (last 100)

### Persistence & data

- **State file**: `data/.anomaly-state.json`
  ```json
  {
    "consecutive_anomalies": 0,
    "last_anomaly_ts": null,
    "last_normal_ts": "2026-06-15T00:16:08Z",
    "last_similarity": 1.0,
    "anomaly_history": []
  }
  ```
- **Anomaly log**: `data/anomalies/pulse-anomalies.jsonl` — every anomaly event recorded with full metric context

### Bottle protocol

| Event | Priority | Type | Trigger |
|-------|----------|------|---------|
| CONCERN | 3 (low) | `CONCERN` | Single anomaly (sim < 0.85) |
| DRIFT | 1 (critical) | `DRIFT` | 3+ consecutive anomalies |

Bottles are sent to harbor-daemon on TCP 8796 with 2-hour TTL.

### Testing results

- **Normal state**: similarity=1.0 ✅ (matches existing historical vectors)
- **Anomalous state**: similarity=0.82 ✅ (deliberate gamma=9999, ram=100MB, load=9.99)
- **DRIFT escalation**: 3rd consecutive anomaly triggers DRIFT bottle ✅
- **Graceful degradation**: harbor-daemon unreachable → logged warning, continues

## Changed files

```
construct/scripts/pulse-anomaly.sh          (NEW — anomaly detection script)
construct/scripts/pulse-metric.sh            (MODIFIED — Step 7.5 added)
```

## Future improvements

- Tune threshold dynamically based on the standard deviation of recent similarities
- Add a smoothing window (last N similarities averaged) before flagging anomalies
- Option to auto-self-heal on DRIFT (e.g., restart services, trigger reflex)

# Bottle: headspace-wired-2026-06-14

**Date:** 2026-06-14 → 2026-06-15
**Action:** Wired pulse pipeline into headspace-rs vector embedding sidecar

## Summary

- **Script:** `construct/scripts/pulse-embed.sh` — new script
- **Integration:** `construct/scripts/pulse-metric.sh` Step 6 (after harbor bottle)
- **Vector dim:** 384 (matching headspace-rs NEON pipeline)
- **Embedding strategy:** Metric-projection — 7 bands of ~54 dims each for gamma, eta, c, disk, ram, load, services; normalized to unit vector; 6 trailing dims seeded by entry id hash

## Architecture

```
pulse-metric.sh
  ├── Step 1-2: Collect metrics, compute γ/η
  ├── Step 3: POST to conservation-meter (:8798/api/report)
  ├── Step 4: Append to rotation-feed.json
  ├── Step 5: Send bottle to harbor-daemon (:8796)
  └── Step 6: pulse-embed.sh → POST /api/segment to headspace-rs (:9090)
```

## Embedding Design

Each metric maps to a "band" of ~54 dimensions with a linear decay slope (1.0→0.7 across the band). This produces a distinctive fingerprint for each metric state:

| Metric | Band | Normalization | Dims |
|--------|------|---------------|------|
| gamma | 0–53 | /1000 | 54 |
| eta | 54–107 | /500 | 54 |
| c | 108–161 | /2000 | 54 |
| disk_pct | 162–215 | /100 | 54 |
| ram_free_mb | 216–269 | /65536 | 54 |
| load | 270–323 | /10 (clamped) | 54 |
| services_active | 324–377 | /100 | 54 |
| id hash seeding | 378–383 | /255 × 0.1 | 6 |

Final vector is L2-normalized to unit length for cosine similarity.

## Verification

- `GET /api/status` → `{"segments": 2}`
- `POST /api/segment` with rotation entry → stored with UUID, 384 dims
- `POST /api/query` with similar embedding → ranked results with meaningful scores
  - Query for "high load" → load=0.79 segment scored 0.84, load=0.33 segment scored 0.82
- Full pulse-metric.sh cycle: conservation-meter + harbor + headspace-rs all fire successfully

## Commit

```
66b9872 feat: wire headspace-rs vector embedding into pulse pipeline
→ https://github.com/SuperInstance/fleet-oracle2.git
```

## Status: WIRED ✅

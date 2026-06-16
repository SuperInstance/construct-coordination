# 🧪 Conservation Meter — Built

**γ + η = C** — the measurable constraint governing the fleet.

## What was built

A Rust daemon at `/home/ubuntu/.openclaw/workspace/construct/conservation-meter/`

### Files

| File | Purpose |
|------|---------|
| `Cargo.toml` | tokio + serde + chrono + clap + axum |
| `src/main.rs` | CLI args, HTTP server (port 8798), inline HTML dashboard, JSON API |
| `src/metrics.rs` | Report struct, ring buffer (VecDeque), burn detection, rolling stats |
| `target/release/conservation-meter` | Compiled binary (~7.1M) |

### Endpoints

- **POST /api/report** — accepts `{"agent","gamma","eta","task","timestamp"}` JSON
- **GET /** — HTML dashboard with Current C, γ/η ratio (color-coded), burn signal, trend sparklines, last 20 table
- **GET /api/status** — JSON of same data

### CLI

```
--port      listen port (default: 8798)
--history   max reports in ring buffer (default: 1000)
--prune-interval  seconds between 24h cleanup (default: 30)
```

### Verified

- ✅ `cargo build --release` — compiles clean, zero warnings
- ✅ Server starts on port 8798
- ✅ POST /api/report accepts and stores metrics
- ✅ GET /api/status returns correct JSON
- ✅ GET / returns styled HTML dashboard with fleet design system colors
- ✅ Burn detection triggers on γ+η with γ rising + η flat (5 consecutive)
- ✅ γ/η ratio color: green <5, yellow 5-15, red >15
- ✅ Sparkline bars for γ, η, C trends
- ✅ Background prune task (24h expiration, 30s interval)

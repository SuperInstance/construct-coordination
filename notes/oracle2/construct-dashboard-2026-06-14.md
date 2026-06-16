# Bottle: Construct Stack Dashboard — 2026-06-14

## Summary
Live fleet dashboard serving on port 8800 with real-time data from all 3 construct daemons.

## Access
- URL: http://localhost:8800/
- API: http://localhost:8800/api/stack-health (aggregated JSON)
- API: http://localhost:8800/api/gc-pid?<disk_pct>

## Cards
1. **Harbor** — 6 bottles stored, live count
2. **Conservation** — γ, η, C, ratio with sparkline of last 10 γ readings
3. **Rotation Feed** — 5 entries, latest with full metadata
4. **GC PID** — 3.46× aggression at 63% disk (from gc-pid-bridge binary)

## Stack Health
- 10 conservation-meter reports, C trend: 974→1134 (trending up, expected with more services)
- Ratio: 1.92 (green, <5)
- All services healthy

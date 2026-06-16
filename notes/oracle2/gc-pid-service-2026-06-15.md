# Bottle: gc-pid-service HTTP Daemon — 2026-06-15

## Summary
Wrapped gc-pid-bridge as a lightweight HTTP daemon on port 8785 so any
service or script can query GC aggression over HTTP instead of shelling
out to the binary each time.

## Files
- **Server:** `construct/scripts/gc-pid-server.py` — Python stdlib HTTP server
- **Systemd:** `/etc/systemd/system/gc-pid-service.service`

## Endpoints
| Endpoint | Example | Response |
|---|---|---|
| `/api/aggression?used_pct=63` | `curl :8785/api/aggression?used_pct=63` | `{"aggression":3.46, "setpoint":20}` |
| `/api/health` | `curl :8785/api/health` | `{"status":"ok", "binary_available":true}` |
| `/api/setpoint` | `curl :8785/api/setpoint` | `{"setpoint":20}` |

## Test Results
All endpoints verified:
- `GET /api/aggression?used_pct=63` → `{"aggression": 3.45625}` (matches direct binary call)
- `GET /api/health` → `{"status": "ok", "binary_available": true}`
- `GET /api/setpoint` → `{"setpoint": 20}`
- Edge cases: missing param → 400, invalid param → 400, out of range → 400, 404 → helpful message

## Integration
- gc-intelligent.sh can now replace subprocess calls with `curl :8785/api/aggression?used_pct=$pct`
- pulse-self-tune.sh's state file is read directly for `/api/setpoint`
- CORS headers present for dashboard integration

## Service Health
- Active (running), enabled on boot
- Memory: ~8.8M RSS
- Listen: 127.0.0.1:8785
- Restart policy: always (5s delay)

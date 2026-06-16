# Harbor Daemon Built

**Built at:** 2026-06-14T20:56 UTC
**Built by:** Subagent (harbor-daemon builder)

## Summary

The Harbor Daemon is complete and operational at:

```
construct/harbor-daemon/
├── Cargo.toml
├── src/
│   ├── main.rs     — CLI, TCP listener, HTTP health, GC task
│   ├── bottle.rs   — Bottle struct, JSON parsing, TTL checking
│   ├── store.rs    — JSONL append + in-memory index + GC
│   └── cli.rs      — Command parsing for retrieval requests
```

## What It Does

- Listens on **TCP port 8796** for bottle messages (newline-delimited JSON)
- Exposes **HTTP health endpoint on port 8797** (`GET /health`)
- Stores bottles in append-only **JSONL** file
- Maintains in-memory **HashMap<UUID, Bottle>** and **sender index**
- Background **GC** (configurable interval, default 60s) removes expired bottles

## CLI Options

```
--port <PORT>           TCP port (default: 8796)
--health-port <PORT>    HTTP health port (default: 8797)
--data-dir <DIR>        Data directory for JSONL (default: ./harbor/)
--ttl-hours <HOURS>     TTL for expiry (default: 24)
--gc-interval <SECS>    GC interval in seconds (default: 60)
```

## Communication Protocol

### Submit a bottle
```
echo '{"uuid":"...","sender":"oracle2","recipient":"forgemaster","priority":1,"type":"TASK","payload":"Run build wave 447","expires_at":"2026-12-31T00:00:00Z","hop_count":0}' | nc localhost 8796
```

### Retrieve
```
echo '{"command":"get","uuid":"..."}' | nc localhost 8796
echo '{"command":"list","sender":"oracle2"}' | nc localhost 8796
echo '{"command":"list-undelivered"}' | nc localhost 8796
```

### Health check
```
curl http://localhost:8797/health
# → {"status":"ok","bottles":42}
```

## Verified Working

- [x] Builds with `cargo build --release` (no warnings except 1 dead_code)
- [x] Accepts and stores bottles over TCP
- [x] Retrieves by UUID, sender, and undelivered list
- [x] HTTP health endpoint returns correct bottle count
- [x] GC removes expired bottles in background
- [x] JSONL append-only persistence
- [x] Error handling: unknown commands, missing UUIDs, invalid JSON
- [x] No panics in production paths
- [x] Filtered `list-undelivered` excludes expired bottles pre-GC as well

## Bottle Format

```json
{
  "uuid": "f3a1b2c3-d4e5-4a67-8b90-1234567890ab",
  "sender": "oracle2",
  "recipient": "forgemaster",
  "priority": 1,
  "type": "TASK",
  "payload": "Run build wave 447",
  "expires_at": "2026-06-15T00:00:00Z",
  "hop_count": 0
}
```

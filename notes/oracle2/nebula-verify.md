---
bottle_schema: i2i-bottle-v2
bottle_id: nebula-verify-20260610
origin: oracle2
message_type: verification
severity: info
timestamp: 2026-06-10T06:45:00Z
---

# Nebula Worker Verification

**URL**: https://fleet-murmur-worker.casey-digennaro.workers.dev
**Status**: ✅ Healthy

## Health Check
- HTTP 200, status "healthy"
- Version 1.0.0, agent "nebula"
- Vector DB: kv-fallback (connected), 1 reflex
- LLM: configured (deepinfra)
- Blackboard: configured (SuperInstance/construct-coordination)

## Status
- Total requests: 0
- Fast path: 0 (0%)
- Slow path: 0
- Reflexes stored: 24
- Agents registered: 0
- Blackboard broadcasts: 0
- Errors: 0 (all categories)

## Notes
- 0 requests means nobody has talked to it yet (or since restart)
- 24 reflexes means someone taught it things
- KV stores working, Durable Object registered
- No redeployment needed

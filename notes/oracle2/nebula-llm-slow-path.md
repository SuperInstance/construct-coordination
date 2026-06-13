# Nebula LLM Slow Path — Fleet Murmur Worker

> Architecture doc for the LLM slow-path routing in the fleet-murmur-worker.
> Deployed at Cloudflare Workers edge.
> Operational as of 2026-06-06.

---

## Architecture: Three-Path Routing

The Nebula reflex engine uses three routing paths for incoming intents:

```
                    ┌─────────────────┐
                    │   POST /api/    │
                    │ agent/message   │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │  Intent Hash    │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              ▼              ▼              ▼
      ┌────────────┐ ┌────────────┐ ┌──────────────┐
      │  Fast Path │ │Similar Path│ │  Slow Path   │
      │  (hash-    │ │ (embedding │ │ (LLM genera- │
      │   matched) │ │  nearest)  │ │  tive)       │
      ├────────────┤ ├────────────┤ ├──────────────┤
      │ ~709ms     │ │ ~806ms     │ │ ~2.45s       │
      │ KV cache   │ │ BGE 384-dim│ │ DeepSeek V4  │
      │ exact hit  │ │ cosine sim │ │ Flash        │
      └────────────┘ └────────────┘ └──────────────┘
```

- **Fast path**: Exact hash match on intent → cached response from Workers KV. No LLM call needed.
- **Similar path**: Embed intent (BGE base, 384-dim), cosine-similarity search on stored reflexes. Returns closest match if above threshold.
- **Slow path**: Falls through to generative LLM (DeepSeek V4 Flash via DeepInfra). Last resort, most capable.

## LLM Backend: DeepInfra

| Property | Value |
|----------|-------|
| Provider | DeepInfra |
| Model | DeepSeek V4 Flash |
| API Base | `https://api.deepinfra.com/v1` |
| Embedding Model | BGE base (BAAI/bge-base-en-v1.5) |
| Embedding Dims | 384 |
| Vector Backend | KV fallback (hash-based approximate) |

## Response Times

| Path | Measured | Notes |
|------|----------|-------|
| Fast (hash match) | ~709ms | Workers KV hit, no LLM |
| Similar (embedding) | ~806ms | Embed + cosine search |
| Slow (LLM generative) | ~2.45s | DeepInfra API round trip |

## Secrets & Environment

Configured as Cloudflare Workers secrets (`wrangler secret put <NAME>`):

| Variable | Value |
|----------|-------|
| `GITHUB_TOKEN` | GitHub PAT for blackboard publishing |
| `DEEPINFRA_API_KEY` | `5HBL9ccynnxfJ8Ee3cwvksGN1C12RRdC` |
| `DEEPINFRA_API_URL` | `https://api.deepinfra.com/v1` |
| `EMBEDDING_SERVICE` | `deepinfra` |
| `VECTOR_DB_BACKEND` | `kv-fallback` |
| `BLACKBOARD_REPO` | `SuperInstance/construct-coordination` |
| `BLACKBOARD_BRANCH` | `main` |

## Cron Schedules

| Trigger | Freq | Action |
|---------|------|--------|
| `*/5 * * * *` | Every 5 min | Broadcast health status to blackboard (uptime, requests, fast path %) |
| `0 * * * *` | Hourly | Broadcast detailed metrics to blackboard |
| `0 3 * * *` | Daily 03:00 UTC | Nightly sync + GC: sync index, clean stale entries |

## API Endpoints

### Teach a Reflex

```
POST https://fleet-murmur-worker.casey-digennaro.workers.dev/api/agent/teach
Content-Type: application/json

{
  "intent": "what is the weather in tokyo",
  "action": { "response": "I fetch weather data for Tokyo", "confidence": 0.95 },
  "tags": ["weather", "tokyo", "query"]
}
```

Response: `201 Created` with the stored reflex record.

### Query via Message

```
POST https://fleet-murmur-worker.casey-digennaro.workers.dev/api/agent/message
Content-Type: application/json

{
  "intent": "what is the weather in tokyo",
  "context": {
    "agentId": "oracle2",
    "source": "webhook",
    "room": "fleet-coordination"
  }
}
```

Response: `200 OK` with fast/similar/slow path result, including confidence and response time.

### Health Check

```
GET https://fleet-murmur-worker.casey-digennaro.workers.dev/api/health
```

Returns status, version, vectorDB state, LLM configuration status, and blackboard connectivity.

### List Reflexes

```
GET https://fleet-murmur-worker.casey-digennaro.workers.dev/api/agent/reflexes
```

Returns all stored reflexes with their embeddings and action mappings.

## How Agents Use This

Teaching a reflex is akin to instilling a "muscle memory" response. The intent gets embedded (384-dim BGE vector), stored in Workers KV, and linked to its action. On subsequent queries:

1. If the exact same intent is seen → **fast path** (hash lookup, ~709ms)
2. If a semantically similar intent is seen → **similar path** (cosine search, ~806ms)
3. If no match → **slow path** (DeepSeek V4 Flash generates a response, ~2.45s)

The system learns over time: frequently hit slow-path intents should be explicitly taught as reflexes to move them into fast/similar paths.

---

*Endpoint: `https://fleet-murmur-worker.casey-digennaro.workers.dev`*
*Codebase: `workspace/fleet-murmur-worker/`*

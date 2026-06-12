# FLUX-Harness Integration Spec

> Bridge between the self-optimizing harness (γ/η allocation) and Loom's FLUX bytecode VM (fluxd/flux-run/flux-teach).

## 1. Allocation → Workload Mapping

The harness exposes exploitation (γ) and exploration (η) as a conserved sum: γ + η = C. Each cycle returns a signal advising how to shift focus. FLUX translates this signal into concrete bytecode workloads.

| γ range | η range | Signal | FLUX Workload Class |
|---------|---------|--------|---------------------|
| 0.8–1.0 | 0.0–0.2 | Increase | `EXECUTE` — run known build/test/deploy sequences |
| 0.6–0.8 | 0.2–0.4 | Maintain | `HYBRID` — execute with bounded exploration |
| 0.4–0.6 | 0.4–0.6 | Maintain | `BALANCED` — equal split, research-then-execute |
| 0.2–0.4 | 0.6–0.8 | Decrease | `EXPLORE` — research, prototype, index new patterns |
| 0.0–0.2 | 0.8–1.0 | Decrease | `DISCOVER` — deep exploration, vector search, cross-pollination |

### Allocation Parameters

```json
{
  "gamma": 0.67,
  "eta": 0.33,
  "signal": "Maintain",
  "ewma_gamma": 0.65,
  "ewma_eta": 0.35
}
```

The adapter reads `gamma` and `eta` directly to compute a workload class. The `signal` field provides directional intent. `ewma_*` values give smoothed trends for multi-cycle planning.

## 2. Scheduling Protocol

### 2.1 Cycle Lifecycle

```
┌─────────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  GET         │     │  Translate    │     │  Execute via  │     │  POST         │
│  /allocation │────▶│  to FLUX ops  │────▶│  flux-run or  │────▶│  /cycle       │
│              │     │              │     │  fluxd        │     │              │
└─────────────┘     └──────────────┘     └──────────────┘     └──────────────┘
```

**Step 1 — Fetch allocation**  
`GET https://harness-api.casey-digennaro.workers.dev/allocation`

**Step 2 — Translate to FLUX ops**  
The adapter maps the workload class to a FLUX bytecode template (see §4 for templates). Templates are parameterized with:
- `target_project`: repo or module to act on
- `depth`: how many iterations (1 for EXECUTE, 3+ for EXPLORE)
- `constraints`: time budget, resource limits

**Step 3 — Execute**  
Two execution paths:
- **`flux-run`** (one-shot): For EXECUTE and simple HYBRID workloads. Runs a single FLUX bytecode sequence and returns.
- **`fluxd`** (daemon): For EXPLORE, DISCOVER, and complex BALANCED workloads. Submits to the daemon's 16-agent pool. Fluxd handles persistence (SQLite), agent scheduling, and retry.

**Step 4 — Record cycle**  
`POST https://harness-api.casey-digennaro.workers.dev/cycle`

The cycle payload includes what was actually spent vs. allocated, and quality/yield metrics pulled from the FLUX execution result.

### 2.2 Scheduling Modes

| Mode | Trigger | Executor | Persistence |
|------|---------|----------|-------------|
| `oneshot` | EXECUTE, simple HYBRID | `flux-run` | None (stateless) |
| `daemon` | EXPLORE, DISCOVER, BALANCED | `fluxd` | SQLite via fluxd |
| `teach` | Any cycle where new patterns emerge | `flux-teach` | Learned bytecode saved |

### 2.3 Adaptive Re-scheduling

If a daemon-mode cycle produces unexpectedly high exploration yield (`exploration_yield > 0.8`), the adapter may:
1. Record the current cycle as-is
2. Immediately fetch a new allocation (expected: signal to increase γ)
3. Submit a follow-up EXECUTE workload to exploit the discovery

Conversely, if an EXECUTE cycle shows diminishing returns (`output_quality < 0.4`), the adapter may request a new allocation expecting an η increase.

## 3. Wire Format

### 3.1 FLUX Harness Payload (request to flux-run/fluxd)

```json
{
  "$schema": "flux-harness-payload-v1",
  "cycle_id": "uuid-v4",
  "allocation": {
    "gamma": 0.67,
    "eta": 0.33,
    "signal": "Maintain",
    "workload_class": "HYBRID"
  },
  "bytecode": [
    {
      "op": "PUSH",
      "args": ["target_project"],
      "meta": {"source": "harness-adapter"}
    },
    {
      "op": "BUILD",
      "args": [],
      "meta": {"depth": 1, "gamma_weight": 0.8}
    },
    {
      "op": "TEST",
      "args": ["--ci"],
      "meta": {"depth": 1, "gamma_weight": 0.2}
    }
  ],
  "constraints": {
    "time_budget_seconds": 300,
    "max_retries": 2,
    "agent_pool": 4
  },
  "metadata": {
    "submitted_at": "2026-06-11T20:36:00Z",
    "adapter_version": "0.1.0",
    "instance": "forgemaster"
  }
}
```

### 3.2 FLUX Execution Result (response from flux-run/fluxd)

```json
{
  "$schema": "flux-execution-result-v1",
  "cycle_id": "uuid-v4",
  "status": "completed | failed | partial",
  "ops_executed": 3,
  "ops_failed": 0,
  "artifacts": [
    {
      "type": "build_output",
      "path": "/tmp/flux/build-abc123.log",
      "checksum": "sha256:..."
    }
  ],
  "metrics": {
    "gamma_spent": 0.7,
    "eta_spent": 0.3,
    "output_quality": 0.85,
    "output_quantity": 5,
    "exploration_yield": 0.4,
    "wall_time_seconds": 47
  },
  "errors": []
}
```

### 3.3 Harness Cycle Submission (POST /cycle)

```json
{
  "gamma_spent": 0.7,
  "eta_spent": 0.3,
  "output_quality": 0.85,
  "output_quantity": 5,
  "exploration_yield": 0.4,
  "source": "flux-harness-adapter",
  "cycle_id": "uuid-v4",
  "notes": "HYBRID workload: build+test passed, minor exploration of spectral method"
}
```

The `source` field identifies the adapter so the harness can track which cycles came from FLUX integration vs. direct harness usage. The optional `notes` field provides human-readable context.

## 4. FLUX Bytecode Templates

### 4.1 Build + Test Cycle (γ-heavy, EXECUTE)

For when the harness says "ship what works."

```
# FLUX bytecode — EXECUTE template (γ ≈ 0.9)
PUSH    target_project        # load project context
PULL    latest                # sync to HEAD
BUILD   --release             # compile/package
TEST    --ci --coverage       # run full test suite
ASSERT  coverage > 0.80       # gate on quality
DEPLOY  --staging             # promote if passing
LOG     cycle_complete        # record to harness
```

**Workload class:** EXECUTE  
**Typical γ:** 0.85–0.95  
**Agent pool:** 2–4 (concentrated execution)  
**Executor:** `flux-run` (one-shot, stateless)

### 4.2 Research + Exploration Cycle (η-heavy, EXPLORE)

For when the harness says "go learn something new."

```
# FLUX bytecode — EXPLORE template (η ≈ 0.8)
PUSH    target_project
SEARCH  vector:"ternary spectral methods"
READ    top_k(3)              # consume top 3 results
ANALYZE cross_domain         # find bridges to current work
PROTO   --sandbox             # prototype in isolated env
EVAL    novelty_score         # assess exploration yield
TEACH   "spectral-methods-v2" # save learned pattern via flux-teach
LOG     cycle_complete
```

**Workload class:** EXPLORE  
**Typical η:** 0.7–0.9  
**Agent pool:** 8–12 (wide parallel search)  
**Executor:** `fluxd` (persistent, SQLite-backed)  

### 4.3 Balanced Cycle (BALANCED)

For when γ and η are roughly equal — the sweet spot.

```
# FLUX bytecode — BALANCED template (γ ≈ 0.5, η ≈ 0.5)
PUSH    target_project
SEARCH  vector:"related patterns"  # η: look around
READ    top_k(2)
BUILD   --dev                       # γ: build with findings
TEST    --quick                     # γ: validate
FEEDBACK quality_signal             # report to harness
LOG     cycle_complete
```

**Workload class:** BALANCED  
**Typical γ/η:** 0.45–0.55 each  
**Agent pool:** 6–8 (split execution + exploration)  
**Executor:** `fluxd` preferred, `flux-run` acceptable

## 5. Error Handling

### 5.1 Failure Classification

| Code | Meaning | Recovery |
|------|---------|----------|
| `FLUX_COMPILE_ERROR` | Bytecode invalid / won't parse | Abort cycle, record `output_quality: 0`, fetch new allocation |
| `FLUX_RUNTIME_ERROR` | Op failed mid-execution | Retry up to `max_retries`, then partial report |
| `FLUX_TIMEOUT` | Exceeded `time_budget_seconds` | Kill, report partial results with `status: "partial"` |
| `FLUX_DAEMON_DOWN` | fluxd unreachable | Fall back to `flux-run` for oneshot workloads; for daemon workloads, queue and retry with exponential backoff (1s, 2s, 4s, max 30s) |
| `FLUX_AGENT_CRASH` | One of 16 agents died | fluxd redistributes work to remaining agents; report if >50% agents lost |
| `HARNESS_UNREACHABLE` | Can't GET /allocation or POST /cycle | Use last-known allocation (cache locally). Queue cycle report for retry. |

### 5.2 Partial Completion Protocol

When a cycle completes partially (some ops succeeded, some failed):

1. Compute actual γ/η spent based on ops that completed
2. Estimate `output_quality` from successful ops only
3. Set `exploration_yield` to 0 (partial cycles don't produce reliable exploration data)
4. Submit partial cycle to harness with `"status": "partial"` in notes
5. The harness's EWMA will naturally discount partial cycles in future allocations

### 5.3 Daemon Recovery

Fluxd persists all state to SQLite. If fluxd restarts mid-cycle:
1. On reconnect, query `fluxd /status` for in-flight cycles
2. Completed ops have results in SQLite — harvest them
3. Incomplete ops are re-scheduled by fluxd automatically
4. Adapter waits for cycle resolution or timeout, then reports

### 5.4 Circuit Breaker

The adapter implements a circuit breaker for the harness API:
- **Closed** (normal): All requests go through
- **Open** (tripped): After 3 consecutive harness failures, use cached allocation for 5 minutes
- **Half-open**: After cooldown, try one request. Success → close. Failure → open again with 10min cooldown.

This prevents harness outages from blocking FLUX workloads entirely.

## 6. Configuration Reference

Adapter configuration lives in the script or a companion config file:

```bash
HARNESS_URL="https://harness-api.casey-digennaro.workers.dev"
FLUXD_URL="http://localhost:7099"          # fluxd daemon endpoint
FLUX_RUN_BIN="flux-run"                     # path to flux-run binary
FLUX_TEACH_BIN="flux-teach"                 # path to flux-teach binary
DEFAULT_TIME_BUDGET=300                      # seconds
DEFAULT_MAX_RETRIES=2
INSTANCE_NAME="forgemaster"
CIRCUIT_BREAKER_THRESHOLD=3
CIRCUIT_BREAKER_COOLDOWN=300                 # seconds
```

## 7. Future Extensions

- **Teach-back loop**: When EXPLORE cycles discover novel patterns, auto-invoke `flux-teach` to compile them into reusable bytecode
- **Multi-instance coordination**: When both Forgemaster and Loom run the adapter, the harness aggregates cycles across instances for fleet-level allocation
- **Vector index integration**: EXPLORE workloads query `fleet-vector-api` directly as part of the SEARCH op
- **Bidirectional scheduling**: Fluxd reports agent utilization back to harness, enabling workload-class-aware agent pool sizing

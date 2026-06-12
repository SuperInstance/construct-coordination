# End-to-End Flow: Bottle → Agent → Conservation Audit

**Status:** Design spec — nothing wired yet.
**Date:** 2026-06-11
**Scope:** How the happy path *should* work once the gaps are closed.

---

## 1. The Happy Path

### Step-by-step trace (one health-check bottle through one ship)

```
Forgemaster (ship) ─── AgentRunner ─── CircuitBreaker ─── Bottle wire ─── Conservation Audit
```

#### Step 1: Forgemaster creates a health-check bottle

The caller (e.g. a Cocapn orchestrator or a cron trigger) constructs a `health.check` request using the **`superinstance-protocol`** crate's `Bottle` type:

```rust
use superinstance_protocol::Bottle;

let request = Bottle::new(
    "cocapn",                          // src
    "ship-alpha",                      // tgt
    "health.check",                    // act
    vec![1, 0, 1],                     // trits (sum = 2)
    &serde_json::json!({               // payload (any Serialize)
        "check_type": "ping",
        "timeout_ms": 5000,
    }),
    300,                               // ttl in seconds
)?;
```

**Wire format produced by `request.encode()`:**

```json
{
  "id": "0196a3b2-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
  "ver": 1,
  "src": "cocapn",
  "tgt": "ship-alpha",
  "act": "health.check",
  "trits": [1, 0, 1],
  "enc": "msgpack",
  "pay": "<base64-of-msgpack({check_type:\"ping\",timeout_ms:5000})>",
  "ttl": 300
}
```

The bottle is now a `Vec<u8>` of JSON on the wire.

#### Step 2: Bottle serialized via protocol (JSON envelope + msgpack payload)

Already done by `Bottle::new()` + `Bottle::encode()`. The JSON envelope is the contract (routable without deserializing the payload). The msgpack payload is opaque until the target decodes it.

A router can inspect routing info via `Bottle::decode_header(&wire)` — returns `BottleHeader` (no payload deserialization).

#### Step 3: Bottle sent to a ship (agent)

This is the **first major gap**. There is no transport layer. The bottle needs to travel from the sender's process to the ship's process. What *should* exist:

```rust
// In a new crate or in superinstance-runtime:
pub trait BottleTransport: Send + Sync {
    fn send(&self, bottle: &[u8]) -> Result<(), TransportError>;
    fn recv(&self) -> Result<Vec<u8>, TransportError>;
}
```

Candidate transports:
- **In-process channel** (`tokio::sync::mpsc`) — for local testing
- **HTTP POST** — for Cloudflare Workers deployment
- **WebSocket** — for persistent connections in the mesh

For the happy path, assume an in-process channel. The Cocapn serializes the bottle and sends it to the ship's receiver channel.

#### Step 4: Ship's circuit-breaker evaluates

Before the agent sees the bottle, a **circuit-breaker** gates it. From `COMPOSITION-PATTERNS.md`, this is the Resilience Pattern:

```rust
// From the composition pattern — not yet implemented as a real crate
pub struct CircuitBreaker {
    state: State,        // Closed | Open | HalfOpen
    failure_count: u32,
    threshold: u32,
}
```

**Check:**
- If `state == Closed` → allow the bottle through to the agent.
- If `state == Open` → check if the ship's `HealthRegistry` says healthy. If yes, transition to `HalfOpen` and allow. If no, reject with an error bottle that **preserves the input trit sum**.
- If `state == HalfOpen` → allow one probe bottle through.

The circuit-breaker does **not** modify trits. It is conservation-transparent.

#### Step 5: Agent receives and processes

The ship uses **`superinstance-agent-trait`**. The wire bytes are decoded into the agent-trait's `Bottle` type (separate from the protocol's `Bottle` — **this is a gap**, see §2). Then:

```rust
use superinstance_agent_trait::{AgentRunner, forgemaster::Forgemaster};

let mut runner = AgentRunner::new(Forgemaster::new());

// Must init first (if not already active)
let init = /* system.init bottle */;
runner.receive(init)?;

// Now process the health check
let response = runner.receive(incoming_bottle)?;
```

The `AgentRunner` enforces:
1. **Lifecycle transitions** — `valid_transition(from, action)` must return `true`
2. **Conservation** — `input.trit_sum() == output.trit_sum()`, else `RunnerError::ConservationViolation`

The `Forgemaster` handles `"health.check"` by producing a response with:
- `act`: `"health.check.response"` 
- `trits`: adjusted to preserve input sum, encoding health as a ternary signal:
  - `+1` → healthy
  - `0` → degraded
  - `-1` → failing

> **Note:** Currently `Forgemaster` only handles `"cycle.request"`, `"system.*"`, and unknown actions. It does not have a `"health.check"` handler. That's a gap.

#### Step 6: Response bottle carries trit state

The response bottle is encoded back to wire format:

```rust
// Response from Forgemaster:
Bottle {
    id: "<new uuidv7>",
    src: "forgemaster",           // flipped
    tgt: "cocapn",                // flipped from input src
    act: "health.check.response",
    trits: vec![1, 1, 0],         // sum still = 2 ← CONSERVED
    payload: msgpack({ health: "healthy", ewma_quality: 0.87 }),
}
```

The response trit sum **must** equal the request trit sum. The `AgentRunner` enforces this at the Rust level. The `audit()` / `audit_strict()` functions in `superinstance-protocol` verify it at the wire level.

#### Step 7: Conservation audit — γ + η = C

After collecting responses from all ships, the Cocapn performs a fleet-wide conservation audit:

```rust
use superinstance_protocol::{audit_strict, Bottle};

fn fleet_audit(request: &Bottle, responses: &[Bottle]) -> Result<FleetHealth, FleetAuditError> {
    // Per-bottle: each response must conserve the request's trit sum
    for response in responses {
        audit_strict(request, response)?;  // Returns Err on violation
    }
    
    // Fleet-level: aggregate health from trit patterns
    let fleet_gamma: i32 = responses.iter()
        .map(|r| r.trits.iter().filter(|&&t| t == 1).count() as i32)
        .sum();
    
    let fleet_eta: i32 = responses.iter()
        .map(|r| r.trits.iter().filter(|&&t| t == -1).count() as i32)
        .sum();
    
    // γ + η + neutral = C (the total trit sum across the fleet)
    let fleet_c: i32 = responses.iter().map(|r| r.trit_sum()).sum();
    
    Ok(FleetHealth {
        gamma: fleet_gamma,
        eta: fleet_eta,
        total: fleet_c,
        ships: responses.len(),
    })
}
```

The conservation law `γ + η = C` holds because:
- Each individual response preserves `trit_sum(request) = trit_sum(response)` (per-ship)
- The sum of all ship γ values equals the fleet's total positive signal
- This is **not** `sum(output_trits) = sum(input_trits)` at fleet level (there are N responses for 1 request); it's that each response individually conserves its share

---

## 2. What's Missing — Gap Inventory

### 2.1 Two Different `Bottle` Types (Critical)

| Crate | `Bottle` type | `id` field | Payload field | Serialization |
|-------|--------------|-----------|---------------|---------------|
| `superinstance-protocol` | `Bottle { id: Uuid, pay: String, ... }` | `uuid::Uuid` | `pay: String` (base64 msgpack) | JSON envelope |
| `superinstance-agent-trait` | `Bottle { id: String, payload: Vec<u8>, ... }` | `String` | `payload: Vec<u8>` (raw bytes) | JSON with b64 serde |

**These are incompatible.** An agent-trait `Bottle` cannot be directly encoded as a protocol `Bottle`. A translation/adaptation layer is needed:

```rust
// Needed: in a bridge crate or in superinstance-runtime
fn protocol_to_agent(proto: &protocol::Bottle) -> agent_trait::Bottle { ... }
fn agent_to_protocol(agent: &agent_trait::Bottle) -> Result<protocol::Bottle, BottleError> { ... }
```

### 2.2 Crate Publish Status

| Crate | Version | Published to crates.io? | Compile status |
|-------|---------|------------------------|----------------|
| `superinstance-protocol` | 0.1.0 | ❌ No | ✅ Compiles (tests pass) |
| `superinstance-agent-trait` | 0.1.0 | ❌ No | ✅ Compiles (tests pass) |
| `superinstance-core` | 0.1.0 | ❌ No | ✅ Compiles (ECS world) |
| `superinstance-runtime` | 0.1.0 | ❌ No | ✅ Compiles (stub) |
| `superinstance-harness` | 0.1.0 | ❌ No | ✅ Compiles (γ/η allocator) |
| `superinstance-foundry` | 0.1.0 | ❌ No | ✅ Compiles (stub `compile()`) |
| `superinstance-embedder` | 0.1.0 | ❌ No | ✅ Compiles (32-dim vectors) |
| `construct-coordination` | 0.1.0 | ❌ No | ✅ Compiles (CoordNode stub) |

**Zero crates are published.** All are `0.1.0` local-only.

### 2.3 Stub-Only Implementations

| Crate | What's Stub | What's Real |
|-------|------------|-------------|
| `superinstance-runtime` | `Runtime` is a `Vec<Workload>` with no async execution, no scheduling, no bottle processing | Type definitions only |
| `superinstance-foundry` | `compile()` returns `Ok(vec![])` unconditionally | `BuildConfig` struct |
| `construct-coordination` | `CoordNode` and `CoordMessage` are plain structs with no network, no bottle integration, no SEED composition | Type definitions only |
| `superinstance-embedder` | Generates embeddings but has no Vectorize ingestion pipeline (only `to_vectorize_json()` string output) | Embedding math, cosine similarity, domain detection |

### 2.4 Missing Integrations (Not Even Stubbed)

1. **Bottle Transport** — No `BottleTransport` trait, no channel/HTTP/WS implementation
2. **Circuit Breaker** — Defined in `COMPOSITION-PATTERNS.md` as a design, no actual crate or module
3. **Health Registry** — Design only in composition patterns, no implementation
4. **Bottle Middleware Pipeline** — `BottleMiddleware` trait and `BottlePipeline` exist only in the doc, not in any crate
5. **Protocol ↔ Agent-Trait Bridge** — No conversion between the two `Bottle` types
6. **Cocapn** — No orchestrator crate. The concept of a "captain" that sends bottles to ships and audits responses doesn't exist as code
7. **Fleet Health Aggregation** — No code to collect N responses and compute fleet-level γ/η/C
8. **`Forgemaster` health.check handler** — Only handles `cycle.request` and `system.*` actions; `health.check` falls through to the unknown action echo
9. **Agent Lifecycle Init Flow** — `AgentRunner` requires `system.init` before processing, but no automated init-on-startup exists
10. **Error Bottle Production** — `COMPOSITION-PATTERNS.md` sketches `BottleError::into_bottle()` but no crate implements it
11. **`superinstance-core` → ECS Agent Storage** — The ECS `World` could store agents as entities with components, but nothing connects it to the agent trait

### 2.5 Missing Tests

- No integration test that sends a protocol `Bottle` through a transport to an `Agent`
- No test verifying conservation across the protocol→agent→protocol round-trip
- No test for fleet-level audit (multiple ships)
- No test for circuit-breaker gating

---

## 3. Implementation Plan — Ordered Steps

### Phase 1: Unify the Bottle (Priority: Critical)

**Goal:** One canonical `Bottle` type that both the protocol and agent trait use.

**Step 1.1:** Decide which `Bottle` wins. Recommendation: **`superinstance-protocol::Bottle`** is the wire format; `superinstance-agent-trait::Bottle` becomes a re-export or alias.

```rust
// In superinstance-agent-trait/Cargo.toml:
[dependencies]
superinstance-protocol = { path = "../superinstance-protocol" }

// In superinstance-agent-trait/src/lib.rs:
pub use superinstance_protocol::Bottle;  // re-export the canonical type
// Remove the duplicate Bottle definition
```

**Step 1.2:** Update `Forgemaster` to use the protocol `Bottle`. Change `payload: Vec<u8>` references to use `Bottle::decode_payload::<T>()` and `Bottle::new()`.

**Step 1.3:** Update `AgentRunner` to use the protocol `Bottle`. The `receive(&mut self, bottle: Bottle) -> Bottle` signature stays the same; only the type changes origin.

**Test:** `cargo test` in both crates. Ensure the existing conservation tests still pass.

### Phase 2: Add `health.check` to Forgemaster (Priority: High)

```rust
// In forgemaster.rs, add to the match in Agent::receive:
"health.check" => {
    let input_sum = bottle.trit_sum();
    let health_trit = if self.ewma_quality > 0.7 { 1i8 }
                      else if self.ewma_quality > 0.3 { 0i8 }
                      else { -1i8 };
    // Build trits that preserve input_sum
    let mut response_trits = vec![health_trit];
    let diff = input_sum - health_trit as i32;
    if diff > 0 { for _ in 0..diff { response_trits.push(1); } }
    if diff < 0 { for _ in 0..(-diff) { response_trits.push(-1); } }

    Bottle::new(
        "forgemaster",
        bottle.src.clone(),
        "health.check.response",
        response_trits,
        &HealthReport {
            status: if health_trit == 1 { "healthy" } else if health_trit == 0 { "degraded" } else { "failing" },
            ewma_quality: self.ewma_quality,
            cycle_count: self.cycle_count,
        },
        300,
    )?
}
```

### Phase 3: Implement BottleTransport (Priority: High)

Create in `superinstance-runtime`:

```rust
pub trait BottleTransport: Send + Sync {
    fn send(&self, wire: &[u8]) -> Result<(), TransportError>;
    fn recv(&self) -> Result<Vec<u8>, TransportError>;
}

// In-process implementation for testing:
pub struct ChannelTransport {
    tx: tokio::sync::mpsc::Sender<Vec<u8>>,
    rx: tokio::sync::Mutex<tokio::sync::mpsc::Receiver<Vec<u8>>>,
}
```

### Phase 4: Implement Circuit Breaker (Priority: Medium)

Create `superinstance-circuit-breaker` crate (or add to `superinstance-runtime`):

```rust
pub struct CircuitBreaker {
    state: BreakerState,
    failure_threshold: u32,
    failure_count: u32,
    success_threshold: u32,
    success_count: u32,
}

pub enum BreakerState { Closed, Open, HalfOpen }

impl CircuitBreaker {
    pub fn gate(&mut self, wire: &[u8]) -> Result<(), BreakerOpen> { ... }
    pub fn record_success(&mut self) { ... }
    pub fn record_failure(&mut self) { ... }
}
```

### Phase 5: Implement Cocapn Orchestrator (Priority: Medium)

Create `superinstance-cocapn` crate:

```rust
pub struct Cocapn {
    ships: Vec<ShipHandle>,
    transport: Box<dyn BottleTransport>,
}

pub struct ShipHandle {
    id: String,
    runner: AgentRunner<Forgemaster>,
    breaker: CircuitBreaker,
}

impl Cocapn {
    /// Send a bottle to all ships, collect responses, audit conservation.
    pub fn fleet_check(&mut self, request: &Bottle) -> Result<FleetHealth, FleetError> {
        let mut responses = Vec::new();
        for ship in &mut self.ships {
            if ship.breaker.gate(request.encode()?).is_err() {
                continue; // skip open-circuit ships
            }
            let wire = request.encode()?;
            self.transport.send(&wire)?;
            let response_wire = self.transport.recv()?;
            let response = Bottle::decode(&response_wire)?;
            ship.runner.receive(response)?; // lifecycle + conservation enforcement
            responses.push(response);
        }
        fleet_audit(request, &responses)
    }
}
```

### Phase 6: Local Integration Test (Priority: High)

```rust
#[test]
fn end_to_end_health_check() {
    // 1. Create 3 ships (Forgemaster agents wrapped in AgentRunner)
    // 2. Init each with system.init
    // 3. Send health.check to each via ChannelTransport
    // 4. Collect responses
    // 5. Verify each response: trit_sum == request trit_sum
    // 6. Verify fleet audit passes
}
```

### Phase 7: Deploy

1. Publish all crates to crates.io (at least `superinstance-protocol` and `superinstance-agent-trait`)
2. Deploy Cocapn as a Cloudflare Worker
3. Each ship runs as a Worker with a `superinstance-agent-trait` agent
4. Bottles travel via HTTP POST between Workers

### Testing Locally Before Deploying

```bash
# In superinstance-cocapn (or a test crate):
cargo test --test end_to_end    # in-process integration test
cargo test --test fleet_audit   # multi-ship conservation test

# For HTTP transport testing:
# Spin up a local HTTP server that receives bottles
# Send bottles via curl and verify responses
```

---

## 4. The Fleet View — How This Scales

### 5 Ships, Each Processing Bottles

```
                    ┌──────────────────────┐
                    │       COCAPN         │
                    │  (fleet orchestrator) │
                    │                      │
                    │  fleet_check() ──────┼───► sends health.check bottle
                    │                      │ ◄─── collects responses
                    │  fleet_audit()  ─────┼───► verifies γ + η = C
                    └──────────┬───────────┘
                               │
              ┌────────────────┼────────────────┐
              │                │                │
     ┌────────▼──────┐ ┌──────▼────────┐ ┌─────▼────────┐
     │  Ship Alpha   │ │  Ship Beta    │ │  Ship Gamma  │  ...
     │               │ │               │ │              │
     │ AgentRunner   │ │ AgentRunner   │ │ AgentRunner  │
     │ <Forgemaster> │ │ <Forgemaster> │ │ <Forgemaster>│
     │               │ │               │ │              │
     │ CircuitBreaker│ │ CircuitBreaker│ │ CircuitBreaker│
     │   [Closed]    │ │   [Open]      │ │  [HalfOpen]  │
     └───────────────┘ └───────────────┘ └──────────────┘
```

### Fleet-Level Conservation

Each ship independently conserves `trit_sum` per the `AgentRunner` enforcement. The fleet view aggregates:

| Ship | Response Trits | Trit Sum | Health Signal |
|------|---------------|----------|---------------|
| Alpha | `[1, 0, 1, 0, 0]` | 2 | +1 (healthy) |
| Beta | *(circuit open, skipped)* | — | — |
| Gamma | `[1, -1, 1, 0, 1]` | 2 | +1 (healthy) |
| Delta | `[0, -1, 1, 1, 1]` | 2 | 0 (degraded) |
| Epsilon | `[1, 0, 1, -1, 1]` | 2 | +1 (healthy) |

**Per-ship conservation:** Each response `trit_sum = 2`, matching the request `trit_sum = 2`. ✅

**Fleet-level metrics:**
- **Fleet γ** (total positive trits): Alpha(2) + Gamma(3) + Delta(3) + Epsilon(3) = **11**
- **Fleet η** (total negative trits): Alpha(0) + Gamma(1) + Delta(1) + Epsilon(1) = **3**
- **Fleet C** (total trit sum): 2 × 4 ships = **8**
- **γ + η + neutrals = total trits**, but **Σ(trit_sums) = 8 = C** ← conserved

### Cocapn's Bird's Eye View

The Cocapn maintains a `FleetHealth` struct:

```rust
pub struct FleetHealth {
    /// Per-ship trit sum (should all equal request.trit_sum())
    pub ship_sums: Vec<i32>,
    /// Total positive signals across fleet
    pub gamma: i32,
    /// Total negative signals across fleet
    pub eta: i32,
    /// Fleet-level constant (sum of all response trit sums)
    pub total_c: i32,
    /// Ships that responded
    pub responding_ships: usize,
    /// Ships skipped (circuit open)
    pub skipped_ships: usize,
    /// Conservation status
    pub conserved: bool,
}
```

The Cocapn can:
1. **Detect degradation** — if `gamma / (gamma + eta)` drops below a threshold
2. **Detect failures** — if a ship's health signal is `-1`
3. **Trigger remediation** — send `system.suspend` to failing ships, `system.resume` when healthy
4. **Report to SuperInstance** — aggregate fleet health bubbles up

### SuperInstance Sees the Whole Mesh

At the highest level, the SuperInstance (the parent system) sees:

```
SuperInstance
  └── Fleet "production"
        ├── Cocapn (captain)
        │     ├── Ship Alpha  ── AgentRunner<Forgemaster> ── γ=0.87
        │     ├── Ship Beta   ── [CIRCUIT OPEN] ── γ=N/A
        │     ├── Ship Gamma  ── AgentRunner<Forgemaster> ── γ=0.91
        │     ├── Ship Delta  ── AgentRunner<Forgemaster> ── γ=0.45 (degraded)
        │     └── Ship Epsilon── AgentRunner<Forgemaster> ── γ=0.93
        │
        └── Fleet Conservation: Σ(trit_sums) = 8, all ships conserved ✅
```

The `superinstance-harness` crate's γ/η allocator can then adjust:
- **Increase η (exploration)** if fleet health is high → try new configurations
- **Increase γ (exploitation)** if fleet health is degraded → stick to known-good patterns
- The `TernarySignal` enum directly maps to bottle trit patterns

### Scaling Beyond 5 Ships

The pattern is additive:
- **N ships** → N independent conservation checks, 1 fleet audit
- **M fleets** → M fleet audits, 1 SuperInstance aggregate
- Each level preserves conservation within its scope
- Circuit breakers prevent cascade failures
- The `BottleMiddleware` pipeline (once implemented) provides composable layers

---

## Summary of What Needs to Be Built

| # | Item | Depends On | Priority |
|---|------|-----------|----------|
| 1 | Unify `Bottle` types (protocol ↔ agent-trait) | — | 🔴 Critical |
| 2 | `Forgemaster` health.check handler | #1 | 🔴 High |
| 3 | `BottleTransport` trait + channel impl | #1 | 🔴 High |
| 4 | `CircuitBreaker` implementation | #1 | 🟡 Medium |
| 5 | `BottleMiddleware` pipeline | #3, #4 | 🟡 Medium |
| 6 | `Cocapn` orchestrator crate | #2, #3, #4 | 🟡 Medium |
| 7 | Fleet audit logic | #1 | 🟡 Medium |
| 8 | Integration tests (e2e) | All above | 🔴 High |
| 9 | Publish crates to crates.io | All above | 🟢 After stable |
| 10 | HTTP transport for Workers deployment | #3 | 🟢 Deploy phase |

**Current state:** 7 crates compile with passing unit tests. Zero crates published. Two incompatible `Bottle` types. No transport, no circuit breaker, no orchestrator, no fleet audit. The pieces exist but nothing is wired together.

**Target state:** One unified `Bottle`, in-process transport, circuit-breaker gating, `Forgemaster` handling health checks, `Cocapn` orchestrating fleet-wide checks, conservation audited at every boundary.

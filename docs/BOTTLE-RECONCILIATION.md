# Bottle Type Reconciliation

## 1. Current State: Incompatibilities

### Bottle struct

| Field | Protocol (`superinstance-protocol`) | Agent-trait (`superinstance-agent-trait`) |
|-------|--------------------------------------|-------------------------------------------|
| `id`  | `Uuid` (uuid crate, UUIDv7)         | `String` (uuid7 `.to_string()`)          |
| `ver` | `u32` — present                      | **missing**                               |
| `src` | `String`                             | `String`                                  |
| `tgt` | `String`                             | `String`                                  |
| `act` | `String`                             | `String`                                  |
| `trits` | `Vec<i8>`                          | `Vec<Trit>` where `Trit = i8`            |
| `enc` | `String` (always `"msgpack"`)       | **missing** — uses serde `#[serde(with)]` |
| `pay` / `payload` | `String` (base64 of msgpack) | `Vec<u8>` (raw bytes, custom b64 serde)  |
| `ttl` | `u32` — present                      | **missing**                               |

### Other type mismatches

| Type | Protocol | Agent-trait |
|------|----------|-------------|
| `BottleHeader` | Exists (id, ver, src, tgt, act, trits, enc, ttl) | **Does not exist** |
| `BottleError`  | Rich error enum (7 variants, `thiserror`) | **Does not exist** |
| `AgentState`   | **Does not exist** | Enum: Init, Active, Suspended, Terminated |
| `AgentReport`  | **Does not exist** | Struct with state, cycle_count, etc. |
| `Agent` trait  | **Does not exist** | `receive(Bottle) -> Bottle` + `inspect() -> AgentReport` |
| `AgentRunner`  | **Does not exist** | Wraps `Agent`, enforces lifecycle + conservation |
| `SystemAction` | **Does not exist** | Enum mapping to `system.*` act strings |
| `Trit`         | Implicit `i8`      | `pub type Trit = i8` |
| `audit`/`audit_strict` | Free functions | Runner enforces inline |
| `Bottle::new`  | Takes trits + `&impl Serialize` payload + ttl, returns `Result` | Takes src/tgt/act only, no payload/trits/ttl |
| `Bottle::new_empty` | Takes trits + ttl | **Does not exist** |
| `Bottle::encode/decode` | JSON wire methods | **Does not exist** (relies on serde) |
| `Bottle::validate` | TTL check | **Does not exist** |
| `derive` traits | Debug, Clone, Serialize, Deserialize | Debug, Clone, Serialize, Deserialize, **PartialEq** |

### Wire format divergence

- **Protocol**: `pay` is a JSON string field containing base64(msgpack(T)). Envelope has `enc` and `ver`.
- **Agent-trait**: `payload` is `Vec<u8>` with custom serde that base64-encodes on serialize. No `enc`, no `ver`, no `ttl`.
- **Result**: The two crates produce **different JSON shapes**. They cannot interop on the wire.

### Dependency divergence

- Protocol depends on: `rmp-serde`, `thiserror`, `serde_json` (explicitly)
- Agent-trait depends on: only `serde`, `uuid`, `base64` (no `rmp-serde`, no `thiserror`)

---

## 2. Unified Types

### Ownership rule

**`superinstance-protocol` owns all wire types** (`Bottle`, `BottleHeader`, `BottleError`, audit functions).
**`superinstance-agent-trait` owns all agent types** (`Agent`, `AgentState`, `AgentReport`, `AgentRunner`, `SystemAction`).
Agent-trait re-exports `Bottle` from protocol.

### Unified `Bottle` (lives in `superinstance-protocol`)

```rust
// superinstance-protocol/src/lib.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Ternary digit.
pub type Trit = i8;

/// Full wire bottle — JSON envelope + msgpack payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bottle {
    pub id: Uuid,
    pub ver: u32,
    pub src: String,
    pub tgt: String,
    pub act: String,
    pub trits: Vec<Trit>,
    pub enc: String,
    /// Base64-encoded msgpack payload.
    pub pay: String,
    pub ttl: u32,
}

impl Bottle {
    /// Create a new bottle with typed payload. Msgpack-encodes `payload` into `pay`.
    pub fn new(
        src: impl Into<String>,
        tgt: impl Into<String>,
        act: impl Into<String>,
        trits: Vec<Trit>,
        payload: &impl Serialize,
        ttl: u32,
    ) -> Result<Self, BottleError> { /* ... unchanged ... */ }

    /// Create a bottle with empty msgpack payload.
    pub fn new_empty(
        src: impl Into<String>,
        tgt: impl Into<String>,
        act: impl Into<String>,
        trits: Vec<Trit>,
        ttl: u32,
    ) -> Self { /* ... unchanged ... */ }

    /// Create a bottle with raw payload bytes (for agent-trait compat).
    /// Msgpack-wraps the bytes, then base64-encodes into `pay`.
    pub fn new_raw(
        src: impl Into<String>,
        tgt: impl Into<String>,
        act: impl Into<String>,
        trits: Vec<Trit>,
        raw_payload: Vec<u8>,
        ttl: u32,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            ver: 1,
            src: src.into(),
            tgt: tgt.into(),
            act: act.into(),
            trits,
            enc: "msgpack".into(),
            pay: B64.encode(rmp_serde::to_vec(&raw_payload).unwrap()),
            ttl,
        }
    }

    // encode, decode, decode_header, decode_payload, validate, trit_sum — unchanged
}
```

`BottleHeader`, `BottleError`, `audit`, `audit_strict` — **unchanged** from current protocol crate.

### Unified agent types (lives in `superinstance-agent-trait`)

```rust
// superinstance-agent-trait/src/lib.rs

// Re-export Bottle and friends from protocol
pub use superinstance_protocol::{
    audit, audit_strict, Bottle, BottleError, BottleHeader, Trit,
};

use serde::{Deserialize, Serialize};
use std::fmt;

/// Lifecycle state of an agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    Init,
    Active,
    Suspended,
    Terminated,
}

/// System lifecycle actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemAction { /* ... unchanged ... */ }

/// Agent inspection report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReport { /* ... unchanged ... */ }

/// The Agent trait — two methods.
pub trait Agent {
    fn receive(&mut self, bottle: Bottle) -> Bottle;
    fn inspect(&self) -> AgentReport;
}

/// AgentRunner — enforces lifecycle + conservation.
pub struct AgentRunner<A: Agent> { /* ... unchanged ... */ }
```

---

## 3. Migration

### Crate dependency graph

```
superinstance-protocol (no agent deps)
       ↑
superinstance-agent-trait (depends on protocol)
```

### Changes to `superinstance-protocol`

| Change | Detail |
|--------|--------|
| Add `PartialEq` derive to `Bottle` | Required by agent-trait tests |
| Add `Trit` type alias | `pub type Trit = i8;` |
| Add `new_raw()` constructor | For agent-trait code that passes raw bytes as payload |
| No other changes | Keep all existing API stable |

### Changes to `superinstance-agent-trait`

| Change | Detail |
|--------|--------|
| Add dep on `superinstance-protocol` | In `Cargo.toml` |
| Remove local `Bottle` struct | Re-export from protocol |
| Remove local `Trit` alias | Re-export from protocol |
| Remove `mod b64` | No longer needed (protocol owns serialization) |
| Remove `uuid` and `base64` deps | No longer needed directly |
| Add `rmp-serde` dep | For `Forgemaster` to construct bottles with typed payloads |
| Update `Forgemaster` | Use protocol's `Bottle` API (`new_raw` or `new_empty`) |
| `Agent::receive` signature | `Bottle` now comes from protocol — same shape, no signature change |

### `Cargo.toml` diff for agent-trait

```toml
# Remove:
uuid = { ... }
base64 = "0.22"

# Add:
superinstance-protocol = { path = "../superinstance-protocol" }  # or versioned
rmp-serde = "1"  # if forgemaster needs typed payloads
```

---

## 4. Breaking Changes & Fixes

### Breaking: `Bottle.id` type changes from `String` → `Uuid` (agent-trait consumers)

**Impact**: All agent-trait code that constructs `Bottle` by hand (Forgemaster, tests) uses `uuid::Uuid::now_v7().to_string()` for `id`.

**Fix**: Use `Uuid::now_v7()` directly (it's already a `Uuid`). All `Bottle { id: ..., .. }` struct literals change from `.to_string()` to raw `Uuid`.

### Breaking: `Bottle` gains `ver`, `enc`, `ttl` fields (agent-trait consumers)

**Impact**: All struct-literal `Bottle { ... }` construction in agent-trait must include these fields.

**Fix**: Use `Bottle::new_raw()` or `Bottle::new_empty()` instead of struct literals. Example forgemaster fix:

```rust
// Before:
Bottle {
    id: uuid::Uuid::now_v7().to_string(),
    src: "forgemaster".into(),
    tgt: bottle.src.clone(),
    act: "system.init.ack".into(),
    trits: bottle.trits.clone(),
    payload: b"initialized".to_vec(),
}

// After:
Bottle::new_raw(
    "forgemaster",
    bottle.src.clone(),
    "system.init.ack",
    bottle.trits.clone(),
    b"initialized".to_vec(),
    300,
)
```

### Breaking: `Bottle.payload` (`Vec<u8>`) → `Bottle.pay` (`String`)

**Impact**: Any code reading `bottle.payload` directly.

**Fix**: Use `bottle.decode_payload::<Vec<u8>>()` for raw bytes, or `bottle.decode_payload::<T>()` for typed data.

### Non-breaking: `Agent` trait signature unchanged

`receive(&mut self, Bottle) -> Bottle` — same types, just the `Bottle` is re-exported.

---

## 5. Test Impact

### Protocol crate tests — **no changes needed**
All existing tests use protocol's `Bottle` directly. `PartialEq` addition is additive.

### Agent-trait crate tests

| Test | Change needed |
|------|---------------|
| All helpers (`sys_bottle`, `cycle_request`) | Rewrite using `Bottle::new_raw` or `Bottle::new_empty` instead of struct literals |
| `forgemaster_handles_cycle_request` | Helper change only; assertions on `.act`, `.src` unchanged |
| `conservation_holds_across_cycles` | `.trit_sum()` still works (re-exported) |
| `agent_runner_enforces_conservation` | Unchanged — uses `Bottle` through re-export |
| `cant_receive_after_terminate` | Unchanged |
| `lifecycle_transitions_are_valid` | Unchanged |

### Forgemaster tests: key pattern

```rust
// Before
fn sys_bottle(action: &str, trits: Vec<Trit>) -> Bottle {
    Bottle {
        id: uuid::Uuid::now_v7().to_string(),
        src: "system".into(),
        tgt: "forgemaster".into(),
        act: action.into(),
        trits,
        payload: Vec::new(),
    }
}

// After
fn sys_bottle(action: &str, trits: Vec<Trit>) -> Bottle {
    Bottle::new_empty("system", "forgemaster", action, trits, 300)
}
```

### Forgemaster impl: key pattern

Every `Bottle { ... }` struct literal in `receive()` becomes a `Bottle::new_raw(...)` call. The `.clone()` of `bottle.src` into `tgt` stays the same. No logic changes.

---

## Summary

One `Bottle` type, owned by protocol, re-exported by agent-trait. Wire format is protocol's format (JSON envelope + base64 msgpack). Agent-trait gains `ver`/`enc`/`ttl` awareness for free. `Agent` trait signature is untouched. Migration is mechanical: replace struct literals with constructors.

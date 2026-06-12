# Bottle Fix Log

**Date:** 2026-06-11
**Status:** ✅ Complete — both crates compile and test green

## What was done

### superinstance-protocol (canonical wire types)

1. **Added `PartialEq` derive to `Bottle`** — required by agent-trait tests for equality assertions
2. **Added `pub type Trit = i8`** — canonical ternary digit alias
3. **Added `Bottle::new_raw()` constructor** — takes `Vec<u8>` raw bytes, msgpack-wraps them, base64-encodes into `pay`. This replaces the agent-trait's old `Bottle { payload: Vec<u8> }` pattern.
4. Added doc comment to `enc` field

No existing API was changed. All 7 original tests pass unchanged.

### superinstance-agent-trait (agent types, re-exports wire types)

1. **Removed local `Bottle` struct** — was a duplicate with incompatible fields (`id: String`, no `ver`/`enc`/`ttl`, `payload: Vec<u8>`)
2. **Removed `mod b64`** — custom base64 serde module no longer needed
3. **Removed local `Trit` alias** — re-exported from protocol
4. **Removed `uuid` and `base64` dependencies** — no longer needed directly
5. **Added `superinstance-protocol` as dependency** — path dependency
6. **Added `serde_json` dependency** — for `AgentReport` serialization
7. **Added re-exports**: `pub use superinstance_protocol::{audit, audit_strict, Bottle, BottleError, BottleHeader, Trit}`
8. **Updated `Forgemaster`**: all `Bottle { ... }` struct literals replaced with `Bottle::new_raw(...)` and `Bottle::new_empty(...)` calls
9. **Updated test helpers**: `sys_bottle()` and `cycle_request()` now use `Bottle::new_empty()`

All 8 tests pass (forgemaster: 7, runner: 1 integrated).

## Dependency graph after fix

```
superinstance-protocol (no agent deps)
       ↑
superinstance-agent-trait (depends on protocol)
```

## One Bottle type

Both crates now share exactly one `Bottle` type, defined in `superinstance-protocol::Bottle` and re-exported by `superinstance-agent-trait`. Wire format is JSON envelope + base64(msgpack) payload with `ver`, `enc`, `ttl` fields.

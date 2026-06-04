# Construct API v2 — Addressing Critical Review Findings

*Based on CRITICAL-REVIEW.md findings. This is the fix plan.*

## The Core Problem (Acknowledged)

The Construct trait requires `std` types everywhere but claims ESP32 compatibility.
This is a real architectural flaw. The fix: **layered traits with compile-time feature gates**.

## v2 Trait Architecture

### Layer 0: BareMetalConstruct (no_std, no alloc)
```rust
// Works on ESP32, no heap, no async
pub trait BareMetalConstruct {
    type Error;
    fn query_lookup(&self, index: u16) -> TritAction;  // O(1) table lookup
    fn capabilities(&self) -> BareMetalCapabilities;     // static, const
}
```

### Layer 1: SyncConstruct (no_std + alloc)
```rust
// Works on Pi, embedded Linux, has heap but no async runtime
pub trait SyncConstruct: BareMetalConstruct {
    fn load_skill(&mut self, id: SkillId) -> Result<(), Self::Error>;
    fn query(&self, q: Query) -> Result<Response, Self::Error>;
    fn shared_state(&self) -> &dyn SharedState;
}
```

### Layer 2: AsyncConstruct (std + tokio)
```rust
// Works on Workstation, DGX, has full async runtime
pub trait AsyncConstruct: SyncConstruct {
    fn request_tool(&mut self, spec: ToolSpec) -> Result<ToolHandle, Self::Error>;
    fn query_stream(&self, q: Query) -> Pin<Box<dyn Future<Output = Response> + Send + '_>>;
}
```

### Feature Gates
```toml
[features]
default = ["std"]
std = ["alloc"]
alloc = []
bare-metal = []  # Only Layer 0
```

## Key Fixes

### Fix 1: Split traits by capability, not by hardware
- `SkillRegistry` — loading/unloading skills
- `ToolRegistry` — requesting/releasing tools  
- `QueryRouter` — routing queries
- `CapabilityReporter` — reporting what's available
- Hardware implementations compose the traits they can support

### Fix 2: No-alloc types for bare metal
- `SkillId` → `&'static str` or `[u8; 32]` instead of `String`
- `Query.payload` → `&[u8]` or `[u8; N]` instead of `Vec<u8>`
- Capabilities → static struct, not runtime `HashMap`

### Fix 3: SharedState needs CRDTs
- `compare_and_swap(key, expected, new)` for Pi↔ESP32
- Vector clocks for ordering across instances
- Eviction policy for bounded memory
- Eventually consistent, not strongly consistent

### Fix 4: ToolFactory uses associated types
```rust
pub trait ToolFactory {
    type Tool: Tool;
    fn create(&self, config: ToolConfig) -> Result<Self::Tool, ToolError>;
}
```

### Fix 5: BrowserConstruct uses wasm-bindgen-futures
- Replace tokio channels with `wasm_bindgen_futures::spawn_local`
- `JsValue` wrapper that's `Send + Sync` via `web_sys::Worker`
- Or: Browser construct only implements SyncConstruct, not AsyncConstruct

### Fix 6: HardwareTier is advisory, not capability ordering
- Remove `PartialOrd` from HardwareTier
- Capabilities are discovered, not assumed from tier
- A browser on a 64GB workstation reports its actual capabilities

## Implementation Priority
1. Refactor ternary-wasm to use Layer 1 (SyncConstruct), not Layer 2
2. Refactor ternary-esp32-firmware to use Layer 0 (BareMetalConstruct)
3. Create `construct-core` crate with the layered traits
4. Update ROADMAP.md with v2 approach
5. Align with CORTEX.json spec from Oracle2

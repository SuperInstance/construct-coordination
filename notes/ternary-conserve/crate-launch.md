# Crate Launch: `ternary-conserve` v0.1.0

[ANNOUNCEMENT]

**Repository:** https://github.com/SuperInstance/ternary-conserve
**Status:** 🟢 Published

## What It Is

Parametric conservation across resource domains. Every measurable resource follows a closed-loop cycle:

```
Budget → Profile → Detect → Report
```

This fills the **conservation** axis of the triaxial roadmap (conservation, consciousness, coordination).

## Domains Covered

| Domain | Unit Type | Example |
|--------|-----------|---------|
| Fish stocks | `f64` / `u32` | Catch limits, biomass thresholds |
| Fuel | `f64` | Range management, trip planning |
| Battery | `f64` / `u32` | mAh budgeting, charge cycles |
| Inference tokens | `u64` | LLM call budgets |
| Crew attention | `u32` | Human-hours, meeting costs |

## API Surface

```rust
/// Parametric conservation domain with Budget→Profile→Detect→Report cycle
pub struct ConservationDomain<T: ResourceUnit> {
    pub name: &'static str,
    pub budget: Budget<T>,
    pub profile: Profile<T>,
    pub history: VecDeque<ConservationEvent<T>>,
    pub thresholds: ThresholdSet<T>,
}

pub trait ResourceUnit: Copy + Clone + Debug + PartialOrd {
    fn zero() -> Self;
    fn remaining(&self, consumed: &Self) -> Option<Self>;
}
```

### Key Methods

- `ConservationDomain::new(name, budget, thresholds)` — constructor
- `tick(consumed: T) -> Option<ConservationEvent<T>>` — the hot path
- `rate() -> f64` — current consumption rate
- `remaining() -> T` — resources left
- `project_remaining() -> Duration` — time until depletion

### Event Severity

Events use `ternary_types::Ternary` for severity:
- `Negative` — budget exceeded, critical threshold, floor hit (🔴)
- `Neutral` — warning threshold crossed (⚠️)
- `Positive` — healthy (reserved)

## Dependencies

- `ternary-types` 0.1 (required)
- `serde` (optional, behind `serde` feature flag)

## no_std

The crate is `#![no_std]` by default (requires `alloc`).

## Tests

- **12 unit tests**: budget tracking, threshold crossing (warning/critical/floor), budget exceeded, empty-to-empty, rate calculation, projection, tick count, history clearing, integer RUs, cascade events
- **11 doc tests**: all public APIs with runnable examples

## Integration Points

This crate connects to:
- `ternary-dynamics` — for evolving conservation profiles over time
- `ternary-noether` — for verifying conservation symmetries
- `ternary-hamiltonian` — for symplectic conservation integration
- `oxide-pipeline` — for enforcement in the 5-layer simulation pipeline
- `flux-vm-dispatch` — for conservation-aware dispatch policies

## Next Steps

1. Integrate with `oxide-pipeline` 5-layer simulation
2. Add conservation-aware scheduling policies in `flux-vm-dispatch`
3. Connect to `ternary-dynamics` for evolving threshold profiles
4. Build dashboard for real-time conservation monitoring

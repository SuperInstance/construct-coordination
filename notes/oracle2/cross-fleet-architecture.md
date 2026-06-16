# Cross-Fleet Architecture: The Ternary Fleet

## A Unified Design Document

**Author:** Oracle2 (SuperInstance Fleet)  
**Date:** 2026-06-16 06:15 UTC  
**Scope:** All 14 ternary crates + protocol + conservation-action

---

## 1. The Ternary Pattern: A Design Philosophy

Every crate in the fleet shares a common DNA: **trits {-1, 0, +1} as universal atomic types**. This isn't a coincidence or a branding choice — it's an architectural conviction that three-valued logic is the correct abstraction for agent coordination.

### Why ternary?

| Binary (0/1) | Ternary (-1,0,+1) | What it means for the fleet |
|---|---|---|
| Yes/No | Harm/Neutral/Help | An agent can **defect** (-1), **abstain** (0), or **cooperate** (+1) |
| True/False | Under/Neutral/Over | A PID controller can **decrease** (-1), **hold** (0), or **increase** (+1) |
| On/Off | Left/Neutral/Right | A route can be **down** (-1), **degraded** (0), or **healthy** (+1) |
| Binary classification | Three-class decision | SVM margin can be **negative**, **zero**, or **positive** |

The philosophical arc: **Binary is for machines. Ternary is for agents.** The third state (0) represents neutrality, abstention, or equilibrium — essential for systems where agents choose not to act.

### The Universal Invariant: γ + η = C

The conservation law is the fleet's equivalent of conservation of energy in physics:

- **γ (gamma):** Total productive content — "how much useful signal does the fleet carry?"
- **η (eta):** Coordination overhead — "how much friction exists between components?"
- **C (constant):** The conserved quantity — "the fleet's total content is invariant"

Every crate either **produces γ, measures η, or verifies C**.

---

## 2. Crate-to-Conservation-Law Mapping

| Crate | Primary Role | γ Mapping | η Mapping | C Mapping | Verification |
|---|---|---|---|---|---|
| **ternary-conserve** | Conservation lifecycle | Resource γ across domains | Conservation overhead | Conserved sum | Self-tests, 12 tests |
| **ternary-svm** | Classification | Decision boundary quality | Margin error | Classification score | 25 tests, CLI broken |
| **ternary-route** | Load balancing | Healthy routes (+1) | Degraded/down overhead | Route capacity | 8 tests, publish-ready |
| **ternary-pid** | Control | Derivative response | Integrator windup | Setpoint tracking | 9 tests, publish-ready |
| **ternary-search-rs** | Vector search | Search relevance | Miss distance | Result quality | 0 tests ❌ |
| **ternary-rhythm** | Temporal patterns | Rhythm coherence | Phase offset | Temporal energy | 52 tests, broken build |
| **ternary-entropy** | Information theory | Shannon entropy H | Redundancy | Total bits | 24 tests, publish-ready |
| **ternary-hamiltonian** | Dynamics | Kinetic energy | Damping loss | Total energy | 30 tests, publish-ready |
| **ternary-fleet-integration** | Bridge crate | Aggregated metrics | Rate limit overhead | Fleet health | 15 tests |
| **ternary-fleet-packing** | Compression | Data density | Packing overhead | Space efficiency | 14 tests |

### Cross-cutting γ/η producers:

| Component | Role |
|---|---|
| **superinstance-protocol** | Wire format — transports γ payloads with trit envelope |
| **superinstance-core** | ECS storage — stores agent components |
| **conservation-languages** | γ+η=C in 9 source languages (proof-by-implementation) |
| **conservation-action** | GitHub Action — blocks CI if γ+η > C |

---

## 3. The Three Bridge Positions

The fleet has a **structural gap**: [math crates] → [bridge crates] → [wire protocol] — the middle is unconnected to the ends.

### Bridge Position 1: Math → Bridge

**Crates involved:** ternary-route, ternary-pid, ternary-entropy, ternary-hamiltonian → ternary-fleet-integration, ternary-fleet-packing

**What should exist but doesn't:**
- A `ConservationMetric` trait that any math crate implements to report γ and η
- A `ConservationAggregator` that sums γ and η across all math crates
- A trit-type adapter that converts between crate-specific trit representations

**Current state:** Each math crate computes its own metrics in its own format. `ternary-fleet-integration` has an `Aggregator` struct but it doesn't know about the math crates.

### Bridge Position 2: Bridge → Protocol

**Crates involved:** ternary-fleet-integration, ternary-fleet-packing → superinstance-protocol

**What should exist but doesn't:**
- `superinstance-protocol::Bottle::from_aggregator(aggregator: Aggregator)` — convert bridge aggregates into protocol bottles
- `Aggregator::from_bottle(bottle: Bottle)` — parse protocol bottles into aggregates
- A `bottle_to_conservation` function that extracts trits and verifies γ+η=C

**Current state:** Bridge produces raw stats. Protocol transports opaque messages. **No serialization adapter exists.**

### Bridge Position 3: Protocol → Action

**Components involved:** superinstance-protocol → conservation-action (GitHub Action)

**What should exist but doesn't:**
- A `conservation-check` binary that reads superinstance-protocol bottles and verifies the conservation law
- A GH Action that runs `conservation-check` on any repo's bottle output
- Integration tests that prove a PR violates conservation → GH Action blocks merge

**Current state:** `conservation-action` is a separate GitHub Action template with no connection to the protocol crate.

---

## 4. The Missing Glue

### Immediate Gaps (Hours to Fix)

| Gap | Location | Fix |
|---|---|---|
| No `Save`/`Load` for Aggregator | `ternary-fleet-integration/src/lib.rs` | Add serde Serialize/Deserialize impls |
| No Bottle conversion for Aggregator | `ternary-fleet-integration` ↔ `superinstance-protocol` | `Bottle::from_aggregator()` fn |
| conservation-action not wired to protocol | `conservation-action/action.yml` | Add `verify_conservation()` step |
| Python bottle client not published | `fleet-oracle2/integrations/` | It exists now (pushed this session) |
| Colony conservation scorer not wired | `colony-games` ↔ `conservation-meter` | Scorer exists (pushed this session), needs daemon |

### Structural Gaps (Days to Fix)

| Gap | Description | Priority |
|---|---|---|
| No runtime conservation meter | The `conservation-meter` port 8798 exists but is a static display, not a live meter | High |
| No ternary-type normalization | 14 crates, 14 different trit representations — no canonical type | Critical |
| No fleet-wide wire format | Colony games, fleet dashboard, pulse all speak different formats | Critical |
| No multi-trial statistical reporting | Darwin experiments need confidence intervals, not point estimates | Medium |
| No TypeScript protocol client | Colony edge emits raw JSON, protocol has Rust+Python clients only | Medium |

---

## 5. How the Pieces Fit Together

```
┌─────────────────────────────────────────────────────────────────────┐
│                        FLEET ARCHITECTURE                           │
└─────────────────────────────────────────────────────────────────────┘
                                                                      
  ┌─────────────┐    ┌──────────────────┐    ┌──────────────────────┐
  │ MATH CRATES │───→│  BRIDGE CRATES   │───→│   WIRE PROTOCOL      │
  │             │    │                  │    │                      │
  │ ternary-    │    │ ternary-fleet-   │    │ superinstance-       │
  │ entropy     │    │ integration      │    │ protocol             │
  │ ternary-    │    │  (aggregates)    │    │  (Bottle envelope)   │
  │ hamiltonian │    │                  │    │                      │
  │ ternary-pid │    │ ternary-fleet-   │    │ Bottle               │
  │ ternary-    │    │ packing          │    │ {id, ver, src, tgt,  │
  │ route       │    │  (compresses)    │    │  act, trits, enc,    │
  │ ternary-svm │    │                  │    │  pay, ttl}           │
  │ ternary-    │    │ ⚠️ GAP: No       │    │                      │
  │ rhythm      │    │ serialization    │    │ ⚠️ GAP: No runtime  │
  │ ternary-    │    │ adapter between  │    │ conservation meter  │
  │ search-rs   │    │ these layers     │    │ reads bottles       │
  └──────┬──────┘    └──────────────────┘    └──────────┬───────────┘
         │                                              │
         │   ┌──────────────────┐                      │
         └──→│ CONSERVATION     │←─────────────────────┘
             │ ACTION (CI/CD)   │
             │                  │
             │ Checks γ+η ≤ C   │
             │ Blocks on vio-   │
             │ lation           │
             └──────────────────┘

  ┌──────────────────────────────────────────────────────────────────┐
  │                    DEPLOYMENT LAYER                              │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                  │
  │  colony-games  ──→  protocol bottles  ──→  conservation-meter   │
  │     (Python)        (JSON+msgpack)         (live γ+η=C display) │
  │                                                                  │
  │  fleet-pulse  ──→  protocol bottles  ──→  fleet-dashboard       │
  │     (TS/CF)         (JSON+msgpack)         (Web UI)             │
  │                                                                  │
  │  delta-clt  ──→  expts directly with LLM agents  ──→ results   │
  │     (Python)         (no bottles yet)          (γ/η scores)      │
  │                                                                  │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 6. Recommendation

If we build one thing to connect the entire fleet, it should be:

> **A `conservation-meter` daemon that listens on a Unix domain socket or HTTP port, receives superinstance-protocol bottles from any component, computes γ+η in real time, and exposes a live dashboard.**

This already exists in skeleton form at port 8798. It needs:

1. **Wire protocol** — accept Bottle objects (JSON+msgpack envelope)
2. **Aggregation** — sum γ and η across all incoming bottles
3. **Persistence** — log γ+η over time for trend analysis
4. **Alerting** — raise a flag if γ+η deviates significantly from predicted C
5. **CI/CD hook** — `conservation-action` calls the meter before deciding yes/no on a PR

The Python client (`superinstance_bottle.py`), the colony conservation scorer (`colony_conservation_scorer.py`), and the integration test (`bottle_integration_test.py`) are all written and pushed to `fleet-oracle2/integrations/`.

---

## 7. Publishing Readiness

| Crate | Status | Metadata Complete | Dependencies Published | Blockers |
|---|---|---|---|---|
| ternary-route | ✅ Ready | ✅ | ✅ | None |
| ternary-pid | ✅ Ready | ✅ | ✅ | None |
| ternary-entropy | ✅ Ready | ✅ | Zero deps | None |
| ternary-hamiltonian | ✅ Ready | ✅ | ✅ | None |
| ternary-conserve | ⚠️ Broken build | Need check | git dep for ternary-types | Missing ternary-types local path |
| ternary-svm | ⚠️ CLI broken | Need check | ✅ | CLI binary won't compile |
| ternary-rhythm | ⚠️ Broken build | Need check | Missing neon-kernel | Dep not in registry |
| ternary-search-rs | ❌ Not ready | Need check | git dep | No tests, hardcoded paths |
| ternary-fleet-integration | ⚠️ No CI/CD | Need check | ✅ | 15 tests pass but no CI |
| ternary-fleet-packing | ⚠️ Deps unpublished | Need check | Git deps | 3 deps not on crates.io |

**Next publish order when token arrives:** ternary-route → ternary-pid → ternary-entropy → ternary-hamiltonian → ternary-fleet-integration

---

## 8. Summary

The ternary fleet is a coherent design philosophy with a structural gap between math crates and protocol. The bridge crates (`ternary-fleet-integration`, `ternary-fleet-packing`) exist but don't connect to either end. The conservation law (γ+η=C) is the unifying invariant — every crate maps to it, but no runtime enforces it.

**Three things to build when Forgemaster returns:**
1. A `ConservationMetric` trait
2. A `Bottle::from_aggregator()` adapter
3. A live conservation meter daemon

All integration work done this session (Python protocol client, conservation scorer, end-to-end test, cross-fleet survey) is pushed to `fleet-oracle2/integrations/`.

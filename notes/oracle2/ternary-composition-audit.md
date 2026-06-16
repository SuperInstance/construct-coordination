# Ternary Composition Audit

**Generated:** 2026-06-10 06:55 UTC  
**Method:** Actual `cargo check` + `cargo test` on 5 crates in workspace  
**Analyzed by:** Cross-crate subagent with Kimi Code (1M+ ctx)  
**Chain:** `ternary-types` → `ternary-core` → `ternary-ring` → `ternary-dynamics` → `ternary-compiler`

---

## 1. Individual Crate Compilation Results

| Crate | Version | Edition | Individual `cargo check` | Tests |
|-------|---------|---------|--------------------------|-------|
| **ternary-types** ✅ | v0.2.0 | 2021 | ✅ (5 warnings) | 30/30 ✅ |
| **ternary-core** ✅ | v0.1.0 | 2021 | ✅ (clean) | 19/19 ✅ |
| **ternary-ring** ✅ | v0.1.0 | 2021 | ✅ (6 warnings) | 21/21 ✅ |
| **ternary-dynamics** ❌→✅ | v0.1.1 (patched) | 2021 | ❌→✅ after fix | 34/34 ✅ |
| **ternary-compiler** ✅ | v0.2.0 | 2024 | ✅ (1 warning) | 60/60 ✅ |

**Total:** 164 tests pass, 0 fail.

---

## 2. Workspace Composition Result ✅

**All 5 crates compile together in a single workspace.** No version conflicts, no feature flag collisions, no type system clashes.

**Workspace Cargo.toml (verified working):**

```toml
[workspace]
resolver = "2"
members = [
    "ternary-types",
    "ternary-core",
    "ternary-ring",
    "ternary-dynamics",
    "ternary-compiler",
]
```

---

## 3. Dependency DAG (As-Is)

```
ternary-types (v0.2.0)
  └─ deps: serde (optional)
  └─ test-deps: serde_json, serde_test, rand
  └─ features: default=["std"], serde, packed
  └─ PUBLIC API: Ternary enum (Negative/Neutral/Positive),
                 TritVector, TernaryMatrix, PackedTrits,
                 TernaryConvertible trait, TernaryError

ternary-core (v0.1.0)   ← ZERO ternary deps!
  └─ deps: NONE
  └─ PUBLIC API: tadd/tsub/tmul/tneg/tinv/tclamp/tdist/tdot (i8),
                 TernaryValue trait (i8, bool impls),
                 TernaryGrid (2D i8 grid),
                 TernaryGraph (adjacency matrix),
                 TernaryDynamics trait,
                 TernaryMeasure trait

ternary-ring (v0.1.0)   ← ZERO ternary deps!
  └─ deps: NONE
  └─ PUBLIC API: Z3(u8) struct (standard 0/1/2 representation),
                 PolyZ3 (Z/3Z polynomials),
                 GF3n (Galois field extension),
                 irreducibility testing

ternary-dynamics (v0.1.0)  ← DEAD DEPENDENCY
  └─ deps (original): ternary-types { git, features=["std"] } → BROKEN
  └─ deps (fixed): ternary-types { path = "../ternary-types" } → unused
  └─ PUBLIC API: DynamicMark, CurveShape, DynamicCurve,
                 DynamicContext, DynamicBalance, DynamicInterpreter
  └─ ACTUAL USES: std::collections::HashMap, std::fmt, f64, String
  └─ NEVER imports ternary_types::* in source code

ternary-compiler (v0.2.0)  ← ZERO ternary deps!
  └─ deps: serde (hard dep)
  └─ edition: 2024
  └─ PUBLIC API: Ternary enum (Neg/Zero/Pos — OWN definition),
                 Op (bytecode ops), Bytecode, Lexer, Parser,
                 Compiler, VM, Optimizer, CFG, dominator_tree
```

---

## 4. Critical Findings

### 🚨 Finding 1: Three Separate Ternary Type Definitions

The ecosystem has **three incompatible ternary value representations**:

| Representation | Crate | Variants | Numeric encoding | Interoperability |
|---|---|---|---|---|
| `enum Ternary` | ternary-types | Negative, Neutral, Positive | -1, 0, +1 (balanced) | Strongly typed, serde |
| `i8` | ternary-core | -1, 0, 1 (raw) | -1, 0, +1 (balanced) | Raw ints, no safety |
| `struct Z3(u8)` | ternary-ring | 0, 1, 2 (modular) | 0, 1, 2 (standard) | Different convention! |
| `enum Ternary` | ternary-compiler | Neg, Zero, Pos | -1, 0, +1 (balanced) | Own enum, different names |

**Impact:** Cannot pass types between crates without manual conversion. The "ternary fleet" cannot share types at compile time.

### 🚨 Finding 2: ternaty-dynamics Has a Dead/Broken Dependency

- **Original:** `ternary-types = { git = "...", features = ["std"] }` 
- **Broken because:** Git fetches `ternary-types v0.1.0` which lacks the `std` feature. The local workspace has v0.2.0 which has `std`, but the git pin ignores it.
- **Worse:** Even after fixing the path, `ternary-dynamics` **never imports or uses any `ternary-types` type**. The dependency is completely unused dead weight.

### 🚨 Finding 3: ternary-core Doesn't Depend on ternary-types (Should)

- `ternary-core` defines `TernaryValue` trait, `TernaryGrid`, `TernaryGraph` using raw `i8`
- These should logically use `ternary_types::Ternary` for type safety
- Currently, `ternary-core` has **zero dependencies** — it's standalone despite being described as "core traits shared across the fleet"

### 🚨 Finding 4: ternary-ring Uses Different Numerical Convention

- `ternary-ring`'s `Z3` uses standard modular arithmetic `{0, 1, 2}`
- `ternary-types` uses balanced ternary `{-1, 0, +1}`
- These are **mathematically different** — Z3:Add in ternary-ring maps 1+1=2≠-1
- A user combining both would get wrong results without explicit conversion

### 🚨 Finding 5: ternary-compiler Duplicates Ternary Enum

- `ternary-compiler` defines its own `Ternary { Neg, Zero, Pos }` with serde derives
- This is **semantically identical** to `ternary_types::Ternary { Negative, Neutral, Positive }`
- Neither depends on the other — pure code duplication

### 🟡 Finding 6: Warning Hygiene

| Crate | Warnings | Severity |
|-------|----------|----------|
| ternary-types | 5 (unused imports, dead code) | Medium |
| ternary-ring | 6 (unused vars, unnecessary mut) | Medium |
| ternary-compiler | 1 (unused var in test) | Low |
| ternary-core | 0 | Perfect |
| ternary-dynamics | 0 | Perfect |

---

## 5. Recommended Dependency DAG (Should-Be)

```
Layer 0: ternary-types
         │
Layer 1: ternary-core ← depends on ternary-types (uses Ternary enum)
         │
Layer 2: ternary-ring ← depends on ternary-core (implements TernaryValue, uses balanced ternary)
         │
Layer 3: ternary-dynamics ← depends on ternary-ring + ternary-core (actually uses types)
         │
Layer 4: ternary-compiler ← depends on ternary-core (uses Ternary via core, removes own enum)
```

**Required changes to achieve this:**

```toml
# ternary-core/Cargo.toml
[dependencies]
ternary-types = { path = "../ternary-types", features = ["std"] }

# ternary-ring/Cargo.toml
[dependencies]
ternary-types = { path = "../ternary-types" }
ternary-core = { path = "../ternary-core" }

# ternary-dynamics/Cargo.toml
[dependencies]
ternary-types = { path = "../ternary-types", features = ["std"] }
# Actually use ternary_types::* in code

# ternary-compiler/Cargo.toml
[dependencies]
ternary-types = { path = "../ternary-types" }
# Remove own Ternary enum, use ternary_types::Ternary
```

---

## 6. Recommended Workspace Cargo.toml (For 189-Crate Fleet)

```toml
[workspace]
resolver = "2"
members = [
    # Layer 0: Foundation
    "ternary-types",
    "ternary-core",
    "ternary-ring",

    # Layer 1: Math foundation
    "ternary-lattice",
    "ternary-permutation",
    "ternary-entropy",
    "ternary-transform",
    "ternary-codes",
    "ternary-regex",
    "ternary-grammar",
    "ternary-compression",
    "ternary-signals",
    "ternary-markov",
    "ternary-automata",
    "ternary-dynamics",
    "ternary-topology",
    "ternary-graph",
    "ternary-projection",
    "ternary-streaming",
    "ternary-noise",
    "ternary-thermodynamics",

    # Layer 2: ML/AI
    "ternary-bayesian",
    "ternary-classifier",
    "ternary-clustering",
    "ternary-trees",
    "ternary-ensemble",
    "ternary-attention",
    "ternary-federated",
    "ternary-transfer",

    # Layer 3: Infrastructure
    "ternary-compiler",
    "ternary-wasm",
    "ternary-protocol",
    "ternary-consensus",
    "conservation-verify",

    # Layer 4: Products
    "ternary-cli",
    "ternary-spreadsheet",
    "ternary-visualizer",

    # ... etc for all 189 crates
]
[workspace.dependencies]
ternary-types = { path = "ternary-types", features = ["std"] }
ternary-core = { path = "ternary-core" }
serde = "1"
```

---

## 7. Cross-Crate Type Compatibility Matrix

```
                     ternary-types   ternary-core    ternary-ring    ternary-compiler
                     ─────────────   ────────────    ────────────    ────────────────
ternary-types        same            i8 (lossy)      u8 (conflict)  own enum (dup)
ternary-core         i8 (lossy)      same            i8 ↔ u8 (conv)  i8 (lossy)
ternary-ring         u8 (conflict)   u8 (needs conv)  same           u8 (needs conv)
ternary-compiler     own enum (dup)  i8 (lossy)      u8 (conv)      same
```

**X marks the spot:** Every crate is incompatible with every other at the type level.  
**Solution:** Unify on `ternary_types::Ternary` as the single source of truth.

---

## 8. Verification Summary

| Metric | Value |
|--------|-------|
| Crates analyzed | 5 |
| Crates that compile individually | 4/5 (80%) |
| Crates that compile in workspace | 5/5 (100%) after dep fix |
| Tests passing | 164/164 (100%) |
| Dead dependencies found | 1 (ternary-dynamics → ternary-types) |
| Type representation conflicts | 3 separate Ternary enums |
| Code duplication | 2 Ternary enums with identical semantics |
| Warnings across fleet | 12 total (all non-blocking) |

### Bottom Line

The crates **do compile together** in a workspace, but the architectural promise of a "ternary fleet" is **not realized at the code level**. The dependency graph is nearly flat, type representations are fragmented, and inter-crate composability is theoretical rather than proven. The recommended fixes are mechanical and well-understood — the ecosystem map already exists as documentation, it just needs to become executable code.

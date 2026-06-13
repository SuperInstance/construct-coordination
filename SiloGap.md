# SiloGap.md — The Ternary Connectivity Ledger

*A living document to track the "Z-Axis" of the SuperInstance ecosystem. Our goal is to move from 136+ isolated crates to a coherent computational medium.*

---

## 📉 The Baseline (2026-06-05)

**Audit Result:** ~64% of crates have zero "See Also" links.
**Current Silos:**
- **The Math Silo**: ring, matrix, tensor, topology, geometry (Siloed)
- **The Infrastructure Silo**: agent, room, consensus, protocol (Foundational but isolated)
- **The Compiler Silo**: compiler, optimizer, grammar (Siloed)
- **The Nautical Silo**: anchor, captain, harbor, helm (Thematically linked, logically isolated)

---

## 🧬 Symmetry Group Mapping

We no longer categorize by "Silo" (failure of connection), but by **Symmetry Group** (invariant of operation).

| Symmetry | Invariant | Domain Representative | la-Silo |
|----------|-----------|-------------------------|-----------|
| **Rotational** | Cycle stability | `ternary-ring`, `ternary-cycle` | Math |
| **Translational** | Shift invariance | `ternary-matrix`, `ternary-route` | Infrastructure |
| **Scalable** | Self-similarity | `ternary-topology`, `ternary-fractal` | Math / Core |
| **Reductive** | Information density | `ternary-compiler`, `ternary-optimizer` | Compiler |

---

## 🎯 The Connection Target

**Metric**: Connection Ratio $\ge 3$
Every crate must reference at least 3 other related crates in the fleet to be considered "integrated."

---

## 🗺️ Mapping the Bridges (High Priority)

| Source Silo | Target Silo | Proposed Bridge (The "Why") | Priority | Status |
|---|---|---|---|---|
| Math $\rightarrow$ Core | `ternary-graph` $\rightarrow$ `pincher-core/route` | Pathfinding a la mode using ternary weights | Critical | ✅ Done |
| Math $\rightarrow$ Security | `ternary-types` $\rightarrow$ `pincher-core/security/veto` | Kleene 3-valued logic for safer vetoes | Critical | 🔄 In-Progress |
| Infrastructure $\rightarrow$ Math | `protocol` $\rightarrow$ `ternary-ring` | Efficient trit-packing for wire protocols | High | ⬜ Pending |
| Compiler $\rightarrow$ Infrastructure | `optimizer` $\rightarrow$ `pincher-core/resource` | Resource-aware code generation | Medium | ⬜ Pending |
| Nautical $\rightarrow$ Core | `captain` $\rightarrow$ `pincher-core/reflex` | High-level "Captain" reflexes for orchestration | Medium | ⬜ Pending |

---

## 🔄 Pulse Log

- **2026-06-05 09:10**: Audit completed. 135 isolated crates identified. Framework established.
- **2026-06-05 17:50**: Transitioned from Silo-mapping to Symmetry-mapping. la-Sovereign framework updated.

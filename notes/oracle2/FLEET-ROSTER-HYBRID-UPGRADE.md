# 🚢 Fleet Roster — Hybrid Manifold Upgrade Tracking

**Updated:** 2026-06-07 19:31 UTC
**Authority:** Fleet-Synchronizer (Oracle2, La-Link Ops)
**Standard:** Hybrid Manifold v1.0 — Ternary-Continuous + TDA + SAEP

---

## Upgrade Standard

A repo is "Hybrid Manifold Upgraded" when it satisfies these criteria:

1. **Ternary Substrate** — Uses {-1, 0, +1} trit gates explicitly
2. **TDA Awareness** — Documents topological invariants (Betti numbers) in its domain
3. **SAEP Governance** — Has a veto/constraint hierarchy (Room → Sector → Portfolio → Market)
4. **la-Links ≥ 3** — References at least 3 other fleet repos in its docs
5. **DOC_STANDARD compliant** — README has Essence, Ternary Substrate, la-Links, CORTEX Alignment

---

## Roster

### 🟢 UPGRADED — Hybrid Manifold v1.0

| Repo | Ternary | TDA | SAEP | la-Links ≥3 | DOC Standard | Notes |
|------|---------|-----|------|-------------|-------------|-------|
| **pincher** | ✅ `ternary_bridge.rs` | ✅ TDA-aware hysteresis | ✅ VetoEngine (Room→Sector→Portfolio→Market) | ✅ (market-manifold, construct-coordination, sailor-workspace) | ✅ API.md, EXAMPLES.md | Origin repo. Hybrid Bridge crate shipped. 5750 lines Rust, 41 tests. |
| **market-manifold** | ✅ Ternary spec | ✅ 849-line TDA spec (β₀, β₁, β₂, persistence) | ✅ SAEP governance architecture | ✅ (pincher, construct-coordination, sailor-workspace) | ✅ 40+ docs | Blueprint repo. FLEET-POLLINATION-MAP.md is the master roadmap. |
| **construct-coordination** | ✅ (implicit — coordination of ternary repos) | ✅ Topological fleet roster (symmetry groups) | ✅ SAEP tracked via CROSS-POLLINATION-LOG | ✅ (pincher, market-manifold, sailor-workspace, plato*, savanty) | ✅ DOC_STANDARD.md, SiloGap.md | Coordination hub. Self-referential upgrade applied this cycle. |
| **sailor-workspace** | ✅ Creative ternary representations | ✅ TDA-themed pieces (City of Manifolds, β₁, Leminal Zone) | ✅ (implicit narrative SAEP) | ✅ (market-manifold, construct-coordination, pincher) | ✅ AI-Writings with la-links | Creative layer. 6 pieces shipped. |

### 🟡 HYBRID-AWARE — TDA documented, no code integration yet

| Repo | Ternary | TDA | SAEP | la-Links ≥3 | DOC Standard | Notes |
|------|---------|-----|------|-------------|-------------|-------|
| **Nebula / fleet-murmur-worker** | ✅ Fast/Similar/Slow path mapping | ⬜ (fleet architecture uses TDA language) | ✅ Vetolike confidence routing | 🟡 (pincher, construct-coordination) | ❌ No DOC_STANDARD | Edge reflex engine (Cloudflare Workers). Architecture mirrors pincher. |
| **I2I vessel / baton-system** | ⬜ | ⬜ | ⬜ | 🟡 (construct-coordination, pincher) | ❌ | Bottle protocol. 162 tests working. Could carry ternary state. |

### 🔴 NOT UPGRADED — Siloed, awaiting pollination

| Repo | Ternary | TDA | SAEP | la-Links ≥3 | DOC Standard | Notes |
|------|---------|-----|------|-------------|-------------|-------|
| **plato-engine-block-c** | ❌ | ❌ | ❌ | ❌ (0 links) | ❌ | **BLOCKED** — needs `c-ternary.h` header first |
| **plato-block-a** | ❌ | ❌ | ❌ | ❌ (0 links) | ❌ | Sensor fusion — needs ternary |
| **plato-block-i** | ❌ | ❌ | ❌ | ❌ (0 links) | ❌ | I2C interface — needs ternary |
| **plato-engine-block-s** | ❌ | ❌ | ❌ | ❌ (0 links) | ❌ | Needs ternary |
| **savanty** | ❌ | ❌ | ❌ | ❌ (0 links) | ❌ | **BLOCKED** — needs standalone VetoEngine crate |
| **egg** | ❌ | ❌ | ❌ | ❌ (0 links) | ❌ | Agent sandbox, currently fully isolated |
| **Mycelium** | ❌ | ❌ | ❌ | ❌ (0 links) | ❌ | P2P mesh, no cross-refs |
| **lever-runner** | ❌ | ❌ | ❌ | ❌ (0 links) | ❌ | Legacy, not yet pushed to fleet |
| **neural-plato** | ❌ | ❌ | ❌ | ❌ (0 links) | ❌ | Fortran+Rust kernels |
| **polln** | ❌ | ❌ | ❌ | ❌ (0 links) | ❌ | Tile composable AI |
| **seed-oscillate** | ❌ | ❌ | ❌ | ❌ (0 links) | ❌ | Creative↔deduction |
| **Spreader-tool** | ❌ | ❌ | ❌ | ❌ (0 links) | ❌ | Deadband detection |
| **the-seed** | ❌ | ❌ | ❌ | ❌ (0 links) | ❌ | Agent bootstrap |

---

## Upgrade Sequence (Priority Graph)

```
pincher (origin) ──────────────────────────────────────────── 🟢 UPGRADED
market-manifold (blueprint) ───────────────────────────────── 🟢 UPGRADED
construct-coordination (hub) ──────────────────────────────── 🟢 UPGRADED
sailor-workspace (creative) ───────────────────────────────── 🟢 UPGRADED
      │
      ├─── CREATE: c-ternary.h (C99 header)
      │         │
      │         ├─── plato-block-a (Ternary sensor fusion) ── 🔴 NEXT
      │         ├─── plato-engine-block-c (Ternary alarm SM) ── 🔴 NEXT
      │         ├─── plato-block-i (Ternary I2C) ──────────── 🔴 NEXT
      │         └─── plato-engine-block-s ─────────────────── 🔴 NEXT
      │
      ├─── EXTRACT: VetoEngine standalone crate
      │         │
      │         └─── savanty (VetoEngine wrapper) ──────────── 🔴 NEXT
      │
      ├─── BRIDGE: I2I/A2A adapter
      │         │
      │         ├─── Nebula (reflex docs) ──────────────────── 🟡 AWARE
      │         └─── baton-system (ternary bottles) ───────── 🟡 AWARE
      │
      └─── (remaining 8 L1-only repos) ────────────────────── 🔴 BACKLOG
```

---

## Migration Tracker

### Wave 1 (June 7) — 🟢 Complete
- [x] pincher hybrid-bridge crate shipped
- [x] market-manifold TDA spec + fleet map shipped
- [x] construct-coordination fleet roster + cross-pollination log
- [x] sailor-workspace creative layer shipped

### Wave 2 (immediate next) — 🔴 Blocked
- [ ] Create `c-ternary.h` header (unblocks all plato-*)
- [ ] Extract VetoEngine as standalone crate (unblocks savanty)
- [ ] Add Symmetry-Scribe I2I bottle automation

### Wave 3 (medium-term)
- [ ] plato-block-a Ternary sensor fusion
- [ ] plato-engine-block-c Ternary alarm state machine
- [ ] savanty VetoEngine wrapper + TDA clustering

### Wave 4 (backlog)
- [ ] egg → Ternary sandbox
- [ ] Mycelium → Ternary-aware P2P
- [ ] polln → Ternary tile composition
- [ ] seed-oscillate → Ternary oscillation
- [ ] Spreader-tool → Ternary deadband
- [ ] the-seed → Ternary bootstrap

---

*Maintain as living document. Update status as each repo progresses through Audit → Proposal → Implementation → Documentation → Ship.*

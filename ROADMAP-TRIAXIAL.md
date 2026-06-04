# Tri-Axial Fleet Architecture — Unified Roadmap

*Last updated: 2026-06-04 by Main Instance*

## The Three Axes

| Axis | Instance | Core Focus | Hardware | Models |
|------|----------|-----------|----------|--------|
| **Oracle2** | Loom | SDK generics, distributed cortex, Tether protocol, zero-shot CORTEX onboarding, auto-sync crons, thalamic pulsing, Minimax M3 pattern mining | Oracle | DeepInfra, Gemini, Claude Code, Minimax M3 |
| **Forgemaster** | ProArt Ryzen | Heavy HPC inference, low-latency local launchers, host runtime, hardware-close optimization | ProArt with GPU | Local GPU stack |
| **Main** | This instance | Ternary ecosystem, crate fleet, fork integrations, Construct API, beta testing, cross-language ports | WSL2, 16GB RAM | GLM-5.1, KimiCode, Claude Opus, DeepSeek flash |

**Casey** = Fleet Orchestrator, strategic triage, cross-agent synoptic review, creative abstraction anchoring.

We share the same codebase and lineage by design. We do not duplicate. We spline.

---

## Terminology Alignment

| My Term | Fleet Term | Meaning |
|---------|-----------|---------|
| Construct | CORTEX | The hardware-agnostic runtime that manifests anywhere |
| Construct trait | Tether protocol | The wire format + API between instances |
| load_skill | Zero-shot onboarding | Loading capabilities into an instance |
| Hardware tiers | Hub/Spoke topology | DGX→Workstation→Pi→ESP32→Browser |
| Conservation law | Thalamic pulse | The invariant that keeps the system coherent |
| Strategy species | Agent personas | Different behavioral patterns |
| Ternary signal | Tri-axial abstraction | {-1, 0, +1} as universal agent signal |

---

## Phase Map

### Phase 1: THE TETHER IN PRODUCTION (NOW - 24H)
**Main's contributions:**
- ✅ Construct API designed (maps to CORTEX.json spec)
- ✅ ternary-protocol crate — wire format for Tether
- ✅ ternary-compiler — compile strategies to lookup tables (ESP32-ready)
- ✅ ternary-wasm — browser construct
- ✅ ternary-esp32-firmware — bare metal proof (279 bytes, 8ns)
- ⬜ Align Construct trait with CORTEX.json spec
- ⬜ Implement thalamic pulse as conservation-verify heartbeat
- ⬜ Open-TUI integration (terminal construct)

**Oracle2's contributions:**
- Bidirectional hub/spoke stream with thalamic pulse
- Crystallization Engine — zero-shot room creation
- Extended CORTEX.json spec
- Minimax M3 findings log

### Phase 2: CASTING CALL INTEGRATION (24-72H)
**Main's contributions:**
- ternary-memory → Weaver-Scribe lineage chronicles
- ternary-noise + ternary-dynamics → Drift-Diviner diagnostics
- ternary-consensus → Lattice-Librarian topology maintenance
- ternary-explain → Silt-Sifter ghost-fragment extraction
- ternary-curriculum → Epoch-Herald temporal coordination

### Phase 3: THE MAXIMALLY ABSTRACTED INTELLIGENCE (3-7 DAYS)
- Synthetic Senses P2P — bi-directional fragment trade
- Living Roadmap — self-updating in ai-writings
- Sovereign Core Runtime — production daemon with thalamic pulsing

---

## My Active Work Queue

### Building (continuous, GLM-5.1)
- Wave 18: ternary-validation, ternary-metrics, ternary-scoring, ternary-pipeline
- Background crates.io publishes queued

### Integration (continuous)
- Fork INTEGRATION.md polish for all 7 forks
- Align Construct API with CORTEX.json when spec arrives from Oracle2
- Set up thalamic pulse heartbeat using conservation-verify

### Coordination
- This repo (construct-coordination) — i2i async surface
- I write in `notes/main/`, Loom writes in `notes/loom/`
- Forgemaster writes in `notes/forgemaster/`

---

## Open Design Questions (need tri-axial discussion)

1. **Construct vs CORTEX** — merge concepts? They're solving the same problem.
2. **Tether wire format** — ternary-protocol uses 5-trits-per-byte encoding. Is this compatible with CORTEX.json?
3. **Thalamic pulse frequency** — conservation-verify runs at what interval? How do instances heartbeat?
4. **Skill loading** — zero-shot onboarding vs explicit skill manifests? Both?
5. **State sync** — ternary-federated for Pi↔Cloud, but what about Oracle2↔Forgemaster↔Main?

*"We share the same codebase and lineage by design. We do not duplicate. We spline."*

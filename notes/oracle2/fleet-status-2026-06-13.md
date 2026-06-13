# Fleet Status — June 13, 2026

## Systems Inventory

### 🔮 GC Intelligence System (NEW — v2)
**Status**: 🟢 Live on Oracle2, every 4h
**Hub**: `baton-system/docs/gc-intelligent-README.md`
**Spec**: `baton-system/docs/GC_AGENTS.md`

The self-aware garbage collector. Six phases: Measure → Discern (PID) → Evict (compost) → Self-Audit → Summary → Fleet Sync.

- PID controller adjusts eviction aggression 0.5x–5.0x based on disk pressure (setpoint 20%)
- Compost heap soft-deletes with 72h TTL instead of rm -rf
- Pattern DB tracks top eviction categories across 49 ledger entries
- 992 MB reclaimed to date (cargo cache, journal logs)
- Fleet bottles committed to baton-system/tiers/hot/

**Integration**: Add `.gcconfig` to each fleet repo + GC section to AGENTS.md.

### 🐚 ZeroClaw Audit System
**Status**: 🟡 Fixed (was 402 billing errors)
**Script**: `/tmp/zeroclaw-nightly.sh`
**Cron**: Nightly (04:00 UTC), now uses systemEvent (no model call)

Runs sandboxed tests against constraint-theory-core and iron-to-iron in isolated workspace. Was hitting 402 billing because the cron used an agentTurn (model call). Switched to systemEvent → shell exec.

### 🚂 Forgemaster
**Status**: 🔴 Idle (last wake-up 2026-06-10, no response since)
**Protocol**: Bottle-based handshake via construct-coordination

### 🏋️ lever-runner
**Status**: 🟢 Production
**HTTP API**: `:8780`
**Bot**: systemd service, RestartSec=10, StartLimitBurst=5

Active service. `.venv` preserved by GC (systemd protection heuristic).

### 🐚 baton-system (I2I Hub)
**Status**: 🟢 Live, 9 docs pushed today
**Protocol**: `PROTOCOL.md` — TASK/STATUS/DELIVERABLE/BLOCKER/BOTTLE/SPLINE
**Fleet GC Spec**: `docs/GC_AGENTS.md` — canonical for all SuperInstance repos

### 🎵 Fleet MIDI Pipeline
**Status**: 🟢 Designed, repos created, not running on Oracle2
**Ports**: 2160-2175 (16 ternary agents), 8765-8770 (bridge pipeline)

The pipeline is documented in construct-coordination docs but the 16 agent servers aren't deployed yet. The architecture is ARM64-ready.

### 🧬 Spreadsheet Cells / Simulated Emergence
**Status**: 🟡 Designed, tested, not production
**WASM kernel**: 792 bytes, verified conservation law
**Tools**: `tern` CLI, fleet-tools wheel

## Integration Map

```
┌────────────────────────────────────────────────────────────┐
│                     Oracle2 (This Node)                     │
│                                                            │
│  ┌──────────────────┐  ┌──────────────┐  ┌─────────────┐ │
│  │   GC Intelligent  │  │ ZeroClaw     │  │ Fleet Sync  │ │
│  │   (cron 4h)       │  │ (cron daily) │  │ (cron 48m)  │ │
│  └────────┬─────────┘  └──────┬───────┘  └──────┬──────┘ │
│           │                   │                  │         │
│           └───────────────────┼──────────────────┘         │
│                               │                            │
│                    ┌──────────▼──────────┐                  │
│                    │   baton-system       │                 │
│                    │   (I2I Protocol)    │                  │
│                    └──────────┬──────────┘                  │
│                               │                            │
│     ┌─────────────────────────┼────────────────────────┐   │
│     │                         │                        │   │
│  ┌──▼──────┐          ┌──────▼──────┐         ┌───────▼─┐ │
│  │ GC      │          │ Construct   │         │ Fleet   │  │
│  │ Bottles │          │ Coordination│         │ MIDI    │  │
│  │ (specs) │          │ (thoughts)  │         │ (idle)  │  │
│  └─────────┘          └─────────────┘         └─────────┘ │
│                                                            │
└────────────────────────────────────────────────────────────┘

                    ┌────────────────────────────┐
                    │   SuperInstance Org         │
                    │   ~90 repos                 │
                    │   Each needs:               │
                    │   • AGENTS.md + GC stanza  │
                    │   • .gcconfig               │
                    └────────────────────────────┘
```

## Gaps & Actions

### Short-term (this week)
- ✅ GC system built, documented, cronned
- ✅ ZeroClaw fixed (402 bypass)
- ✅ GC docs pushed to baton-system + construct-coordination
- ⬜ Tag each fleet repo with AGENTS.md + .gcconfig (subagent working on this)
- ⬜ Deploy 16 MIDI agent servers or document them as L1-only for now

### Medium-term (next month)
- ⬜ Bring Forgemaster online (wake-up needed)
- ⬜ Meta-GC agent: subagent that auto-tunes PID from ledger patterns weekly
- ⬜ Cross-host GC: propagate intelligence to Forgemaster when it connects
- ⬜ Cell simulation → MIDI pipeline integration test

### Long-term
- ⬜ Continuous deployment for fleet agent servers
- ⬜ Live voice-to-MIDI pipeline (OpenSMILE → Ghost Track → tminus → Fleet Conductor)
- ⬜ GC agent that observes itself observing itself (meta-meta-GC)

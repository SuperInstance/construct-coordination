## GC Intelligence System Deployed

**Node**: Oracle2  
**Date**: 2026-06-13  

The self-aware GC system is live. Key additions to the fleet:

### What's New
- **`scripts/gc-intelligent.sh`** — PID-controlled garbage collector that adjusts eviction aggression based on disk pressure (0.5x–5.0x multiplier)
- **`scripts/gc-predictor.py`** — Python analytics engine that reads the JSONL ledger and computes burn rate, trend, and time-to-critical predictions
- **Compost Heap** — soft-delete with 72h TTL instead of immediate `rm -rf`
- **`gc-pin` Registry** — per-path protection manifest (immortal/hot/warm/cold)
- **Self-Audit** — validates data stores, prunes own logs, flags critical disk

### Docs
- Fleet GC spec: `baton-system/docs/GC_AGENTS.md`
- System docs: `baton-system/docs/gc-intelligent-README.md`
- Bottles at: `baton-system/tiers/hot/gc-intelligence-bottle.md`

### Runbook
```bash
./scripts/gc-intelligent.sh --status    # dry-run, see what would happen
./scripts/gc-intelligent.sh --execute   # normal cycle (runs every 4h via cron)
./scripts/gc-intelligent.sh --deep      # aggressive, also clears caches
./scripts/gc-intelligent.sh --audit     # analyze past GC patterns
./scripts/gc-intelligent.sh --calibrate # auto-tune PID controller
```

### Next Steps
1. Roll out `.gcconfig` files to each fleet repo declaring GC tier/protection
2. Create weekly deep-clean cron
3. Build meta-GC agent that auto-adjusts thresholds

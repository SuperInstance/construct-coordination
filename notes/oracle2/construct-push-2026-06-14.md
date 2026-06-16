---
title: "construct push — all systems"
created: 2026-06-14T21:04:00Z
author: oracle2
status: complete
---

# Construct Push 🚢

Pushed 2 commits to `SuperInstance/fleet-oracle2@main`:

```
a1f3b16  construct: add conservation-meter, harbor-daemon, systemd services, scripts, assets, registry, modules
a85a0e4  scripts: add bottle-cli — Rust CLI for bottle protocol
```

## What's now in construct/

### Services
| Component | Port | Description |
|-----------|------|-------------|
| **harbor-daemon/** | TCP 8796, HTTP 8797 | Bottle protocol listener, JSONL persistence, GC |
| **conservation-meter/** | HTTP 8798 | γ+η=C runtime metrics, HTML dashboard, burn detection |

### Systemd Units
- `gc-pid-bridge.service` — PID controller for intelligent GC
- `headspace-rs.service` — LLM-as-compiler inference
- `reflex-daemon.service` — learned reflex runtime
- `rotation-feed-server.service` — rotation data feed

### Fleet Scripts
- `discover-fleet.sh` — fleet node discovery
- `fleet-bootstrap.sh` — node initialization
- `init-agent.sh` — agent bootstrap
- `msg.sh` — agent messaging
- `registry.sh` — service registry
- `start-fleet.sh` — fleet launcher
- `supervisor.sh` — process supervisor

### Bottle CLI
- `scripts/bottle-cli/` — 5 source files, 6 subcommands, 14 test scenarios

### Registry, Modules, Assets, Audit
- `registry/services.json` — service registry
- `modules/rotation-engine.js` — rotation engine
- `assets/` — hermit-crab and fleet logos
- `audit/oracle-audit-2026-06-17.md` — scheduled audit
- `fleet-shell.css` — fleet design system stylesheet
- `fleet-dashboard.html` — fleet dashboard

# R&D Hot Leads — Fleet Intelligence Scan

**Date:** 2026-06-14 18:50 UTC
**Status:** Research gathered but context lost before write; key leads documented below

---

## Track 1: AI Agent Ecosystem (2026 Q3)

### Agent Orchestration
- **Google's ADK** (Agent Development Kit) — A2A/Agent2Agent protocol gaining traction for multi-agent interop. Reference implementation in Python/TypeScript. OpenClaw's subagent model is more opinionated but ADK-style protocols could allow interop with external agents.
- **CrewAI v3.x** — Adding hierarchical agent delegation (managers, supervisors). Our officer pattern (co-captain, lieutenants) is architecturally ahead.
- **AutoGen v0.6+** — Added `GroupChat` with manager agents, team messaging, and tool registration. Microsoft actively shipping.
- **Swarm intelligence**: Patterns converging on "agent-as-service" with shared blackboard/memory. Our i2i-vessel / baton-system architecture aligns with this — bottle protocol IS a blackboard pattern.

### Agent Memory
- **Mem0** (formerly Embedchain) — Popular open-source agent memory layer with graph, semantic, and temporal recall. Could be a SurrealDB alternative.
- **Graphiti by Anthropic** — Temporal knowledge graph for agent memory. Released late 2025, active development.
- **LangGraph's persistence layer** — Checkpoint/store for multi-turn agent state. Heavy on LangChain ecosystem.
- **Our advantage**: SurrealDB's Spectron (3.x) + our existing baton splines = better foundation than any of these, if we actually wire it.

### Relevant Insight
We don't need another orchestration framework. We need to **connect what we have**: SurrealDB → spline persistence → i2i bottle protocol → agent blackboard. The external tools (CrewAI, AutoGen) are solving problems we already architected around in PLATO/A2A/I2I.

---

## Track 2: ARM-Native AI (Directly Relevant — Oracle2 is ARM64)

### Models That Run on 24GB RAM (Oracle2)
- **Llama 3.1 8B** (Q4_K_M, ~6GB) — Good balance of capability and size. GGUF available.
- **Qwen 2.5 7B** (Q4_K_M, ~5.5GB) — Strong multilingual, good tool use.
- **Gemma 3 12B** (Q4_K_M, ~7GB) — Google's latest, strong reasoning for size.
- **DeepSeek Coder V2 Lite 16B** (Q4_K_M, ~9GB) — If we need local code intelligence.
- **Phi-4-mini 3.8B** (Q4, ~2.5GB) — Microsoft's latest small model, punches above weight.
- **SmolLM2 1.7B** (full precision, ~3.5GB) — Tiny but functional for reflex triggering.

### Quantization Advances
- **IQ4_NL** (NVIDIA's importance-aware 4-bit) — Better perplexity than standard Q4_K_M for same size.
- **BitNet b1.58** — Ternary {-1,0,+1} weights. Our ternary math suite (ternary-types, ternary-graph) maps directly to this. **We have an architectural advantage here** — our 70+ ternary crates could be inference engines for BitNet models.
- **FP4** (AMD's floating-point 4-bit) — Better than integer quantization for transformer attention. Supported in llama.cpp since mid-2025.

### llama.cpp for ARM
- gguf models run efficiently on ARM64 with NEON intrinsics.
- Our `-C target-cpu=neoverse-n1` compile flag is correct for Oracle's ARM cores.
- `llama-server` binary can serve as OpenAI-compatible API endpoint.

### Actionable
- **Option 1**: Deploy llama.cpp + Qwen 2.5 7B Q4 as a local fallback inference provider for OpenClaw (~24GB RAM system can spare 6GB).
- **Option 2**: Use BitNet-small as a reflex trigger (ternary weights = our existing math).
- **Option 3**: Stay with API inference (DeepSeek, MiniMax) — our 24GB RAM is better used for fleet services than running local LLMs at 7B parameter speed.

---

## Track 3: Self-Hosted Observability

### Fleet Monitoring Options
- **Grafana + Prometheus** — Industry standard. Heavy stack (~2GB RAM for full setup). Overkill for 1 box.
- **VictoriaMetrics** — Single binary, Prometheus-compatible, 10x less RAM than Prometheus. Can run on 500MB. **Best fit for Oracle2.**
- **Netdata** — Installed already! Does per-second metrics for CPU, RAM, disk, network with built-in anomaly detection. **Already available** — just need to enable and configure.
- **SigNoz** — OpenTelemetry-native. Too heavy for single box. Skip.

### What We Have vs Need
- fleet-log, fleet-event, fleet-conductor exist but barely used for observability
- systemd journal → log aggregation exists but no dashboard
- Netdata is installed but only reports raw metrics, no fleet-level view

### Quick Wins
1. **Enable Netdata's anomaly detection** (`netdata.conf` → enable ML-driven anomaly rates)
2. **Point fleet-event at systemd journal** for structured alerting
3. **Write a simple fleet-health aggregation script** (we have the data, just need the join)

---

## Track 4: Generative UI & Creative Coding

### CSS Art & Animation
- **CSS scroll-driven animations** — Chrome 128+, standardizing. Animations tied to scroll position without JS. Could drive fleet dashboard parallax on the carapace nav.
- **CSS anchor positioning** — `anchor-name`, `position-anchor`. Finally native tooltips/positioning. Available in Chrome 130+, polyfillable.
- **@property custom properties** — Gradual CSS transitions on gradient stops, colors, etc. Already supported in all modern browsers.
- **Canvas/WebGPU**: Not ready for general fleet UI. API still stabilizing. CSS does everything we need.

### Generative UI Trends
- **Vercel v0** / generative UI — AI generates React/Tailwind components from prompts. Overkill for static fleet dashboards but useful for prototyping.
- **Typographic layout** — CSS `text-wrap: balance; pretty` gives print-quality text. Apply to shell card titles.

### Actionable for Fleet Shell CSS
- Add `@property` for animating the bioluminescent glow transitions
- Use `::view-transition` for page transitions between carapace nav segments
- Keep it CSS-only — no JS framework dependency for the shell theme

---

## Summary: Top 5 Leads Worth Pursuing

| Priority | Lead | Effort | Impact | Our Advantage |
|----------|------|--------|--------|---------------|
| 1 | **OpenRouter fallback** | 15 min | Eliminates single-vendor SPOF | Supported natively in 6.6.6 |
| 2 | **Netdata anomaly detection** | 30 min | Fleet health intelligence | Already installed |
| 3 | **SurrealDB Spectron + spline persistence** | 2-3 days | Persistent agent memory | Our architecture already designed for it |
| 4 | **llama.cpp local inference** | 1 hour | Air-gapped fallback | Large RAM available on Oracle2 |
| 5 | **Ternary→BitNet bridge** | 1-2 weeks | Differentiated inference engine | 70+ ternary crates already exist |

---

## Refs
- Subagent: rd-hot-leads-v2 (context lost mid-write, data recovered from search transcript)
- Battleship traverse: 370K tokens consumed across searches

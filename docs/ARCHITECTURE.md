# Fleet Architecture — June 2026

## System Overview

```
Voice/Text → OpenSMILE Bridge (:8765) → Ghost Track (:8767) → tminus (:8768)
    → Fleet Conductor (:8769) → 16 Ternary Agents (:2160-2175)
        → WebSocket Broadcast → MIDI Notebook Bridge → NotebookLM (:8080)
        → Piper TTS (:8770)
        → Cell-to-Fleet Bridge → Agent Analysis Loop
```

## Core Components

### Fleet Conductor (:8769)
Central routing hub. Routes incoming tasks to one or more ternary agents via:
- `/dispatch` — MIDI cue dispatch to named agents
- `/think` — Generic reasoning task, parallel fan-out to targets
- `/agents` — Fleet health status
- WebSocket broadcast — pushes events to connected bridges

### 16 Ternary Agents (:2160-2175)
Specialized reasoning modules, each with dual-mode operation:
- **cue mode** (MIDI): process note arrays, emit ternary vectors → MIDI
- **think mode** (text): analyze natural-language input, return domain-specific assessment + ternary vector

| Port | Agent | Domain | Cue | Think |
|------|-------|--------|-----|-------|
| 2160 | chord | Harmony | Inversion, structure | Partition, composition |
| 2161 | scale | Progression | Scale step, direction | Hierarchy, ordering |
| 2162 | voicing | Texture | Spread, cluster | Arrangement, density |
| 2163 | tempo | Timing | Rate, subdivision | Pacing, rhythm |
| 2164 | cc | Control | Parameter modulation | Control-flow analysis |
| 2165 | expression | Affect | Intensity*, emotion | Affective mapping |
| 2166 | dynamics | Energy | Volume, accent | Energy assessment |
| 2167 | pan | Space | Stereo, spread | Spatial distribution |
| 2168 | modulation | Motion | Speed, depth | Change detection |
| 2169 | arp | Sequence | Order, direction | Sequential reasoning |
| 2170 | groove | Rhythm | Syncopation, feel | Tension, resolution |
| 2171 | velocity | Attack | Strike force, accent | Impact assessment |
| 2172 | fx | Texture | Effect, density | Transformation, layer |
| 2173 | register | Range | Octave, position | Scope, scale |
| 2174 | melody | Line | Contour, interval | Narrative, succession |
| 2175 | bass | Foundation | Root, step pattern | Grounding, support |

### Cell-to-Fleet Bridge
Connects cell simulation emergent behavior to ternary agent analysis:
1. `spreadsheet-cells` model runs TE-weighted cells in 3 topologies (ring, random, full)
2. Cell values quantized to ternary {-1,0,+1}
3. Conservation check: Σ vector, detect drift
4. Dispatch to conductor `/think` → 5 domain agents analyze from different perspectives

### WASM Ternary Kernel (ternary-core.wasm, 792 bytes)
Pure WebAssembly implementation of the core ternary→MIDI mapping:
- `mapping(ptr, len, accumulator)` — cumulative note generation (note += val*4)
- `processOne(val, accumulator)` — single-step processing
- `conservation(ptr, len)` — sum of ternary values
- `normalized_conservation(ptr, len)` — drift detection (|sum|*2 ≤ len)
- `mapping_old(ptr, len)` — backward compatibility wrapper

### I2I Bridge Protocol
Translates between fleet bottle protocol and spreadsheet cell formulas:
- STATUS → =STATUS()
- BLOCKER → =IF() (with `to` field preserved in third arg)
- SPLINE → =INVARIANT()
- SYNTHESIS → =COMPOSE()
- Full round-trip verification with `to` field preservation

## Tools

### tern CLI (`~/oracle2/bin/tern`)
Rust ternary math toolkit with subcommands:
- `tern vec` — encode, sum, dot, magnitude, quantize
- `tern cell` — run cell simulations in ring/random/full topologies
- `tern analyze` — talk to fleet conductor
- `tern pipe` — stdin processing pipeline
- `tern doc` — philosophy and agent documentation

### Fleet Tools (`~/oracle2/fleet-tools/`)
Python CLI toolkit installable via `pip install -e .`:
- `fleet-health` — agent status dashboard, conservation checks
- `bottle` — I2I bottle reader/writer/inbox
- `agent-aggregate` — parallel think/synthesize across 16 agents
- `wheel-cli` — Wheel of Creation cycle management

### Science Documentation (`docs/science/`)
- `THEORY.md` — Complete ternary algebra, TE-weighting, emergence theory
- `CONSERVATION.md` — Formal proof and generalization of Σ law
- `SPATIAL_MATH.md` — Eisenstein integers, Pythagorean triples, pincher roadmap

## Wheel of Creation (`~/oracle2/wheel.sh`)
4-phase continuous innovation cycle:
1. **Ideation** → fork conductor for creative prompts
2. **R&D** → use Claude/Kimi for experiments
3. **Implementation** → agent dispatches, code generation
4. **Beta Testing** → cross-architecture verification (ARM64 × x86_64)

## Conservation Law
Σ(Δ_midi) = 4 × Σ(ternary) — formalized and verified with normalized drift detection.

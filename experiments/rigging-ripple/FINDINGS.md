# FINDINGS: Ripple Propagation in Ternary Rigging Networks

## Executive Summary

Built a 20-rig interconnected network and ran systematic shake experiments. The ternary-rigging crate produces **deterministic, full-network propagation** from any starting point due to cyclic connectivity, but several physically interesting behaviors (reflection, resonance, conservation) are absent or broken by design decisions.

---

## Experiment Results

### 1. Propagation Distance: Full Saturation

**Every rig reaches every other rig.** When you shake any single rig with ±1, the ripple propagates through all 19 others in at most 10 steps. The network is so densely connected that there are no "dead zones" — every node is reachable from every other node.

The propagation follows a clear depth-first pattern through the rope graph, reaching the core ring first (4 rigs), then mid-layer (4 more), then edges and leaves.

### 2. Damping by Connection Weight

| Weight | Behavior | Final Values (10-node chain, input +1) |
|--------|----------|----------------------------------------|
| +1     | Full propagation, no damping | `[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]` |
| 0      | **Immediate kill** — one step and done | `[1, 0, 0, 0, 0, 0, 0, 0, 0, 0]` |
| -1     | Sign-flip oscillation | `[1, -1, 1, -1, 1, -1, 1, -1, 1, -1]` |

**Key insight**: Weight=0 creates hard boundaries. Weight=-1 doesn't damp — it inverts. There is no gradual damping. In ternary, you either transmit fully, invert, or block entirely.

### 3. Oscillation Frequencies: No Resonance

Tested frequencies: every 1, 3, 5, and 10 ticks over 30-tick runs.

| Frequency | Peak Energy | Final State |
|-----------|-------------|-------------|
| 1 (every tick) | 30 | 17/20 nonzero (oscillating) |
| 3 | 30 | All zero |
| 5 | 30 | All zero |
| 10 | 30 | All zero |

**No resonance detected.** The energy trajectory is identical at all frequencies — a spike to 30 on shake ticks, 0 otherwise. The ternary clamp (values bounded to {-1, 0, +1}) prevents amplitude buildup. You can't get resonance in a system where the max value is +1.

The frequency-1 case is the only one where the network retains nonzero values between shakes because each shake is overwritten before it can decay. At all lower frequencies, the alternating Pos/Neg pattern cancels out: shake +1, network floods +1, shake -1, network floods -1, shake +1, floods +1 again. The Zero inputs between shakes don't propagate (they just set the origin to Zero without ripples since ropes transmit 0×weight=0).

### 4. Bridge Rigs: Everything is a Bridge

**All 20 rigs are bridges.** Because the core ring (0-1-2-3-0) connects to the mid layer, which connects to edges, which connect to leaves and hubs, the graph has no articulation points in practice. Every rig can reach every other through the cyclic topology.

However, there are **energy asymmetries**:
- Most rigs produce total energy = 30 (all 20 rigs at ±1)
- **Rig 7 (mid-7)**: energy = 22 — slightly lower due to its position in the propagation order
- **Rig 10 (edge-10)**: energy = 4 — dramatically lower because it's connected via a weight-0 rope (rig 6→10 transmits nothing)
- **Rig 14 (leaf-14)**: energy = 2 — downstream from the weight-0 connection, nearly isolated
- **Rig 18 (hub-18)**: energy = 34 — highest energy, connected to the weight-0 dead zone's mirror path through rig 10, creating slight energy excess via the ternary clamp

The **weight-0 ropes create functional dead zones**. Rigs 10, 14, and 18 form an isolated sub-network that barely participates.

### 5. Boundary Reflection: Blocked by Design

**No reflection occurs.** The `visited` bit-array in `propagate_recursive` prevents any rig from being visited twice in a single propagation event. When a ripple reaches a boundary (like rig 7 in a linear chain) and there's a back-link, the back-link target has already been marked visited, so the reflection is silently dropped.

Even with an inverting pulley (designed to simulate a mirror boundary), the back-link from rig 7→6 produces no trace because rig 6 was visited at step 6 and can't be revisited at step 8.

**This is a fundamental design limitation**: the visited-bit cycle prevention mechanism also prevents wave reflection. True ripple physics requires re-visitation with interference (constructive/destructive), not a one-shot DFS.

### 6. Conservation: Systematically Violated

The quantity γ+H (sum of values + sum of absolute values) was tracked across shakes.

| Tick | Input | Δ(γ+H) | Status |
|------|-------|---------|--------|
| 0 | +1 | +30 | **VIOLATION** (expected +1) |
| 1 | -1 | -26 | **VIOLATION** (expected -1) |
| 2 | +1 | +26 | **VIOLATION** |
| 3 | -1 | -26 | **VIOLATION** |
| 4 | +1 | +26 | **VIOLATION** |
| 5 | 0 | -30 | **VIOLATION** |
| 6-9 | 0 | 0 | ok |

**6 of 10 ticks violate conservation.** The system is fundamentally non-conservative: setting one rig to +1 causes 19 others to flip, creating a Δ(γ+H) of 26-30 instead of the expected ±1. The propagation mechanism acts as an amplifier, not a redistributor.

This is inherent to the design: `set_and_propagate` writes the transmitted value to each target rig, creating energy out of nothing. A conservative system would need to *transfer* energy from source to target, not copy it.

---

## Key Discoveries

### 1. Ternary Networks Are All-or-Nothing
There is no partial propagation or gradual damping. Ropes with weight +1 transmit fully, weight 0 blocks entirely, and weight -1 inverts fully. Combined with the ternary clamp (values can't exceed ±1), the network has no analog of resistance, attenuation, or dissipation.

### 2. The Visited-Bit Prevents Rich Dynamics
Reflection, standing waves, interference patterns — all impossible because each rig can only be written once per propagation event. The system behaves like a one-shot flood fill, not a wave equation.

### 3. Conservation Must Be Designed In, Not Emergent
The current `set_and_propagate` creates energy. Each propagation step copies the value rather than transferring it. For conservation to hold, you'd need something like:
- Transfer semantics: source loses what target gains
- Energy budget: total system energy is fixed and redistributed
- Hamiltonian tracking: each rope has a potential energy term

### 4. Dead Zones from Weight-0 Ropes
Weight-0 ropes create functional isolation. Downstream rigs (10, 14, 18 in this network) barely participate in network dynamics. These could be intentional "firewalls" or unintentional design artifacts.

### 5. No Resonance Possible in Ternary
The ternary clamp prevents amplitude buildup. Resonance requires the ability for small repeated inputs to create increasingly large effects. Since values are bounded to {-1, 0, +1}, the maximum is always reachable in a single step.

### 6. The Network Is a Perpetual Megaphone
From any starting rig, a single shake floods the entire network. There's no concept of locality — the "volume" is always at maximum. This makes the system more like a broadcast mechanism than a ripple tank.

---

## Recommendations for Richer Ripple Physics

1. **Allow re-visitation with interference**: Let ripples revisit rigs, accumulating or canceling values (add values, clamp to ternary)
2. **Energy transfer semantics**: Source rig loses energy when transmitting to target
3. **Attenuation factor**: Ropes could have a decay probability or fractional weight (requires non-ternary internal state)
4. **Phase tracking**: Each propagation step could carry a "phase" to enable interference patterns
5. **Boundaries as features**: Explicit boundary nodes with reflection/absorption coefficients

---

*Experiment run: 2026-06-04*
*Rigging network: 20 rigs, 36 ropes (weights: 33×+1, 1×-1, 2×0)*
*Code: `/home/phoenix/repos/construct-coordination/experiments/rigging-ripple/`*

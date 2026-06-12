# Spatial Math for Pincher

> **Eisenstein Integers, Pythagorean Triples, and the Path to Spatial Cells**  
> *Identifying and filling the spatial math gap in pincher-core*

---

## 1. The Gap: Pincher Has Zero Spatial Math

### 1.1 Current State

An audit of `pincher-core` (`/home/ubuntu/.openclaw/workspace/pincher/pincher-core/src/`) reveals:

| Module | Has Spatial Math? | Notes |
|--------|------------------|-------|
| `capability` | ❌ | Manifest/permission system only |
| `carapace` | ❌ | Shell abstraction, no geometry |
| `db` | ❌ | SQLite database, no spatial queries |
| `dynamics` | ❌ | Veto engine only |
| `embed` | ❌ | Cosine similarity on flat vectors |
| `immunology` | ❌ | Pattern matching, no geometry |
| `intent` | ❌ | Intent parsing, no spatial |
| `migration` | ❌ | Agent migration, no spatial |
| `reflex` | ❌ | Reflex engine, no geometry |
| `resource` | ❌ | Resource budgeting, no spatial |
| `route` | ❌ | Graph routing (generic, not spatial) |
| `rpc` | ❌ | RPC protocol |
| `sandbox` | ❌ | Sandboxing |
| `security` | ❌ | Access control |
| `shell` | ❌ | Shell interface |
| `types` | ❌ | Gestrolith/CRDT types only |

**Total: 0/16 modules have any spatial math.** This is a known gap identified by Forgemaster.

Meanwhile, the downstream crate ecosystem already has spatial primitives that pincher doesn't use:
- `eisenstein-quantize` (10/10 tests) — A₂ hexagonal lattice quantization
- `pythagorean48` (7/7 tests) — Zero-drift vector directions
- `ternary-spatial` (15/15 tests) — P48 + Eisenstein combined spatial queries

### 1.2 Why Pincher Needs Spatial Math

Pincher is a "vector database as runtime" — but its vector math is currently limited to:
- **Cosine similarity** in `embed::cosine_similarity` (flat Euclidean)
- **Graph routing** in `route::shortest_paths` (discrete topology)

It cannot:
- Represent positions in hexagonal (A₂) lattices
- Compute distances in ternary coordinate systems
- Find shortest paths on hexagonal grids
- Quantize continuous positions to hex grid coordinates

These are essential for:
- **Spatial agent positioning** — agents at coordinates, not just IDs
- **Hex map navigation** — moving through A₂ lattice worlds
- **Neighborhood queries** — find agents within hex distance $r$
- **Tensor product operations** — combining decision spaces geometrically

---

## 2. Eisenstein Integers

### 2.1 Definition

The **Eisenstein integers** are the ring:

$$\mathbb{Z}[\omega] = \{a + b\omega \mid a, b \in \mathbb{Z}\}$$

where $\omega = e^{2\pi i / 3} = -\frac{1}{2} + \frac{\sqrt{3}}{2}i$ is the primitive cube root of unity, satisfying $\omega^2 + \omega + 1 = 0$ and $\omega^3 = 1$.

### 2.2 Why Eisenstein Fits Ternary {-1, 0, +1}

The three cube roots of unity are:

$$\{1, \omega, \omega^2\} = \{1, e^{2\pi i/3}, e^{4\pi i/3}\}$$

In the complex plane, these form an equilateral triangle centered at the origin — representing three evenly-spaced directions. This is the **geometric analog** of the ternary alphabet $\{-1, 0, +1\}$:

| Ternary | Geometric Meaning | Complex Representation |
|---------|------------------|----------------------|
| $+1$ | Choose | $1$ (0°) |
| $0$ | Unknown / Center | $0$ (origin) |
| $-1$ | Avoid | $-1$ (180°) |

But the **hexagonal lattice** (A₂) has 6 directions, not 3. The mapping between ternary (3 states) and Eisenstein (6 neighbors) uses:

$$\begin{align}
+1 &\mapsto \{1, \omega^2\} \quad \text{(two "choose" directions)} \\
-1 &\mapsto \{\omega, -1\} \quad \text{(two "avoid" directions)} \\
0 &\mapsto \{0\} \quad \text{(center / indecision)}
\end{align}$$

**Three ternary states → six hexagonal directions**, with each decision mapping to a pair of opposite directions.

### 2.3 The A₂ Hexagonal Lattice

The set of all Eisenstein integers $\mathbb{Z}[\omega]$ forms the **A₂ hexagonal lattice** — the densest packing of circles in 2D.

**Norm:** For $\alpha = a + b\omega \in \mathbb{Z}[\omega]$:

$$N(\alpha) = \alpha \overline{\alpha} = a^2 - ab + b^2$$

This gives the squared distance from the origin in the A₂ lattice.

**Units:** The 6 units of $\mathbb{Z}[\omega]$ are $\pm 1, \pm \omega, \pm \omega^2$ — exactly the 6 directions of the hex grid.

### 2.4 Relationship to Existing Crate

The `eisenstein-quantize` crate (at `/home/ubuntu/.openclaw/workspace/pincher` via its registry) implements A₂ lattice quantization with 10/10 tests passing. The key function is:

```rust
/// Quantize a float (x, y) to the nearest Eisenstein integer.
pub fn quantize_to_eisenstein(x: f64, y: f64) -> (i64, i64) {
    // Project to Eisenstein basis and round
    let a = (x - y / 3.0_f64.sqrt()).round() as i64;
    let b = (2.0 * y / 3.0_f64.sqrt()).round() as i64;
    (a, b)
}
```

The basis vectors are:
- $e_1 = (1, 0) \mapsto 1$
- $e_2 = (1/2, \sqrt{3}/2) \mapsto \omega$

This is the gap: pincher-core has no such function, and no way to use it in agent positioning.

---

## 3. Pythagorean Triples in Ternary

### 3.1 The P48 Group

The `pythagorean48` crate (7/7 tests) defines the **P48 group** — the 48 rotational symmetries of the cube, used to generate zero-drift vector directions.

A **Pythagorean triple** is a triple $(a, b, c)$ of positive integers satisfying $a^2 + b^2 = c^2$. The P48 group takes these triples and uses them to generate direction vectors that have no systematic drift (the "zero-drift" property in `pythagorean48`).

### 3.2 The (3, 4, 5) Triple in Ternary Space

The canonical Pythagorean triple $(3, 4, 5)$ maps to ternary space as follows:

$$3^2 + 4^2 = 5^2$$

In ternary coordinates, represent the vector components as ternary values:

$$\mathbf{v} = (\text{sgn}(3), \text{sgn}(4), \text{sgn}(5)) = (+1, +1, +1)$$

But this loses the magnitude information. The **ternary surface** approach:

Define the **ternary surface** of a Pythagorean triple $(a, b, c)$ as the set of points on the unit sphere corresponding to the direction of $(a, b, c)$:

$$S(a, b, c) = \left( \frac{a}{c}, \frac{b}{c}, 0 \right) = \left( \frac{3}{5}, \frac{4}{5}, 0 \right) \quad \text{for } (3,4,5)$$

### 3.3 Quantized Pythagorean Triples

Map the continuous triple to ternary via threshold quantization ($\theta = 0.75$, from THEORY.md §5):

$$Q(S(3,4,5)) = (Q(0.6), Q(0.8), 0) = (0, +1, 0)$$

This tells us: in the ternary representation of the (3,4,5) surface point, the $b$ component registers as $+1$ (choose), the $a$ component falls in the deadband ($0$ = uncertain), and the $c$ component (scaled to 0) is neutral.

### 3.4 The Pythagorean Surface Map

The 48 elements of P48 generate 48 distinct Pythagorean-triple directions. Mapping each through $Q$ gives a **ternary surface** — a discrete representation of direction in $\mathbb{T}^3$. The `pythagorean48` crate uses this to ensure that any sequence of ternary decisions has zero net drift (the "conservation of direction" property).

---

## 4. The Spatial Cell

### 4.1 Definition

A **spatial cell** is a cell (as in `spreadsheet-cells/cell_simulator.py`) that has a **position** in addition to a **value**:

```rust
struct SpatialCell {
    id: u64,
    position: EisensteinCoord,  // A₂ lattice coordinate (a, b)
    value: f64,                  // Cell value in [-1, 1]
    ternary: Ternary,            // Quantized value in {-1, 0, +1}
    neighbors: Vec<(u64, f64)>,  // (cell_id, TE_weight)
    phase: f64,                  // Oscillator phase for coupling
}
```

where `EisensteinCoord` is:

```rust
struct EisensteinCoord {
    pub a: i64,  // Coefficient of 1 in ω-basis
    pub b: i64,  // Coefficient of ω in ω-basis
}
```

### 4.2 Spatial Cell Operations

A spatial cell supports:

1. **Distance**: $d(c_1, c_2) = \sqrt{N((a_1 - a_2) + (b_1 - b_2)\omega)} = \sqrt{(a_1 - a_2)^2 - (a_1 - a_2)(b_1 - b_2) + (b_1 - b_2)^2}$

2. **Neighborhood**: $N_r(c) = \{c' \in C \mid d(c, c') \leq r\}$ — cells within hex distance $r$

3. **Gradient**: $\nabla v(c) = \sum_{c' \in N_r(c)} w(c, c') \cdot (v(c') - v(c)) \cdot \hat{u}_{c \to c'}$ where $\hat{u}_{c \to c'}$ is the unit direction from $c$ to $c'$ in the A₂ lattice

4. **Laplacian**: $\Delta v(c) = \sum_{c' \in N_1(c)} (v(c') - v(c))$ — the discrete Laplacian on the hex grid

5. **Convolution**: $(K * v)(c) = \sum_{c' \in N_r(c)} K(d(c, c')) \cdot v(c')$ — kernel-weighted spatial averaging

### 4.3 Spatial TE

Transfer Entropy between spatial cells becomes **spatiotemporal**:

$$TE_{c_1 \to c_2} = \sum_{v_2(t+1), v_2(t), v_1(t)} p(v_2(t+1) \mid v_2(t), v_1(t-d_{12})) \log_2 \frac{p(v_2(t+1) \mid v_2(t), v_1(t-d_{12}))}{p(v_2(t+1) \mid v_2(t))}$$

where $d_{12} = d(c_1, c_2)$ is the spatial distance and $t - d_{12}$ accounts for propagation delay. This is **action-at-a-distance with latency**: information from a nearby cell arrives faster than from a distant cell.

### 4.4 Spatial Emergence

With 16 spatial cells on a hexagonal lattice, the emergent patterns from THEORY.md §4 gain a spatial dimension:

1. **Phase waves**: Oscillator synchronization propagates across the lattice as a wave
2. **Topological defects**: Cells at lattice positions where phases can't align create persistent patterns
3. **Clusters as regions**: The five strategy species occupy contiguous hexagonal regions
4. **Boundary effects**: Cells at lattice edges have fewer neighbors, producing edge-dominant (Explorer) vs. interior (Marksman) species distributions

---

## 5. Implementation Roadmap

### 5.1 Phase 1: Eisenstein Core (Week 1)

**Add to pincher-core** a new module `spatial`:

```rust
// pincher-core/src/spatial/mod.rs
pub mod eisenstein;
pub mod pythagorean;
pub mod cell;
pub mod distance;
```

Files:

- `eisenstein.rs`: EisensteinInteger type, A₂ lattice operations, quantization from float → Eisenstein
- `pythagorean.rs`: P48 group, Pythagorean triple generation, ternary surface mapping
- `cell.rs`: SpatialCell type, neighborhood queries, gradient/Laplacian operators
- `distance.rs`: Hex distance, norm computation, bounded-distance queries

**Tests**: 15 minimum (matching `ternary-spatial` crate's 15 tests).

### 5.2 Phase 2: Integration with Fleet (Week 2)

1. **Bridge `spatial` to `route`**: Spatial routing graph on A₂ lattice, using existing `route::shortest_paths`
2. **Bridge `spatial` to `embed`**: Cosine similarity in Eisenstein coordinates
3. **Bridge `spatial` to `reflex`**: Location-dependent reflexes (agent responds differently based on spatial context)
4. **Bridge `spatial` to `types`**: SpatialCell as a new CRDT type (OrMap with spatial key)

### 5.3 Phase 3: Fleet-MIDI Spatial (Week 3)

1. **Spatial panning**: Use spatial cell positions to derive MIDI pan parameters (replacing `pan` agent's current flat ternary)
2. **Hex chorus**: Multiple spatial cells at different hex positions produce chorused MIDI events
3. **Spatial conservation**: Extend CONSERVATION.md to include spatial distance × ternary magnitude

### 5.4 Phase 4: Hex Lattice Explorer (Week 4)

The `hex-lattice-explorer` (deployed at GitHub Pages) is already available at `SuperInstance/hex-lattice-explorer`. Phase 4 adds:

- Spatial cell overlay (show cells on hex grid)
- TE edge display (weighted connections between cells)
- Conservation law animation (watch MIDI = 4× ternary in real time)

### 5.5 crate Impact

| Crate | Status | Role | Spatial Ready? |
|-------|--------|------|---------------|
| `eisenstein-quantize` | ✅ 10/10 | A₂ quantization | ✅ Yes |
| `pythagorean48` | ✅ 7/7 | Zero-drift vectors | ✅ Yes |
| `ternary-spatial` | ✅ 15/15 | Combined spatial | ✅ Yes |
| `deadband-snr` | ✅ 10/10 | Threshold | N/A |
| **pincher-core** | ⚠️ 0/0 | Spatial gap | ❌ No |
| `conservation-matrix` | ✅ 21/21 | Conservation law | N/A |
| `avoidance-cascade` | ✅ 57/57 | Avoidance dynamics | N/A |

**The gap is clear**: three child crates have spatial math; pincher-core itself has none. The fix is to import `eisenstein-quantize` and `pythagorean48` as dependencies and wrap them in a `spatial` module.

### 5.6 Cargo.toml Changes

```toml
[dependencies]
eisenstein-quantize = "0.1"
pythagorean48 = "0.1"

[lib]
path = "src/lib.rs"
```

Then in `lib.rs`:

```rust
pub mod spatial;  // New module wrapping eisenstein-quantize + pythagorean48
```

---

## 6. Worked Example: Spatial Cell Grid

### Setup
Place 16 spatial cells on a $4 \times 4$ hex grid (hexagonal coordinates):

```
(0,0)  (1,0)  (2,0)  (3,0)
(0,1)  (1,1)  (2,1)  (3,1)
(0,2)  (1,2)  (2,2)  (3,2)
(0,3)  (1,3)  (2,3)  (3,3)
```

TE weights are assigned by distance: $w_{ij} = e^{-d(c_i, c_j)/\lambda}$ where $\lambda = 2$ (correlation length).

### Neighbors
For cell $(1,1)$, the hex neighbors within distance 1 are:

- $(0,1)$, $(2,1)$, $(1,0)$, $(1,2)$, $(0,2)$, $(2,0)$

Yes — on a hex grid, each interior cell has 6 neighbors (not 4 as on a square grid). This is the densest packing property of A₂.

### Quantization
Let cell $(1,1)$ have value $v = 0.82$. With $\theta = 0.75$:

$$Q(0.82) = +1 \quad \text{(choose)}$$

Its neighbor $(0,1)$ has value $v = 0.3$:

$$Q(0.3) = 0 \quad \text{(uncertain)}$$

The TE weight from $(0,1)$ to $(1,1)$:

$$w = e^{-d((0,1), (1,1)) / 2} = e^{-1/2} = 0.607$$

This is the same as the current flat cell TE weight, but now it has a **geometric interpretation**: it's the distance-decayed influence on the hex lattice.

---

*References:*
- `/home/ubuntu/.openclaw/workspace/pincher/` — Pincher repo
- `/home/ubuntu/.openclaw/workspace/pincher/pincher-core/src/` — 16 modules, 0 spatial
- `/home/ubuntu/.openclaw/workspace/spreadsheet-cells/cell_simulator.py` — Cell model
- `/home/ubuntu/.openclaw/workspace/fleet-agent/fleet-agent.py` — Agent behaviors
- [Eisenstein integers — Wikipedia](https://en.wikipedia.org/wiki/Eisenstein_integer)
- [A₂ lattice — Hexagonal lattice](https://en.wikipedia.org/wiki/Hexagonal_lattice)
- P48 group — `pythagorean48` crate documentation

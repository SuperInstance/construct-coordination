# Strategic Plan — SuperInstance Construct Ecosystem

*Written: 2026-06-04. Internal use only. Not marketing.*

---

## Preface

This plan is written from the CRITICAL-REVIEW's position: the Construct API
as currently spec'd is a fantasy. The 2,956 lines of Rust read as rigorous
but collapse on contact with hardware reality. A strategic plan that ignores
this is a strategy for building on sand.

The good news: 58+ tested crates, 87+ repos, and 1700+ tests constitute a
real body of work. The foundation is real. The integration layer is broken
and fixable. That's a better position than "nothing works."

---

## 1. Critical Path Analysis

### What MUST happen first

The entire ecosystem hangs on one thing: **the Construct API must actually
work on the hardware it claims to support.** Until `EspConstruct` compiles
on xtensa without heap allocation and `BrowserConstruct` doesn't use tokio
in WASM, every "integration" built on top of the current spec is a façade.

**The blocking chain:**

```
CRITICAL-REVIEW findings (known)
│
└─► Construct API v2 spec
    │   (Split CoreConstruct/SyncConstruct/AsyncConstruct)
    │   (Fix UB in wire protocol trit packing)
    │   (Real security model, even MVP tier)
    │
    └─► construct-core crate
        │   (BareMetalConstruct: no_std, no alloc, const dispatch)
        │   (SyncConstruct: std, no async)
        │   (AsyncConstruct: tokio, full power)
        │
        ├─► ternary-protocol v2 alignment
        │   │   (UB-free trit packing)
        │   │   (Oracle2 Tether/CORTEX.json mapping)
        │   │
        │   └─► Cross-instance wire format (all 3 instances can talk)
        │
        ├─► EspConstruct (verified no_std, 8ns, real hardware)
        │
        ├─► BrowserConstruct (wasm-bindgen, no tokio, real WASM)
        │
        ├─► PiConstruct (async, cloud proxy with real circuit breaker)
        │
        └─► Fork integration (2-3 forks, not all 7)
            │
            ├─► hermit-claw (already current, closest to us)
            ├─► open-terminal (392 lines of integration already written)
            │
            └─► THE DEMO (cross-tier live demo)
                │
                └─► Mantality SDK v1.0 (developer-facing macro layer)
                    │
                    └─► SHIP
```

### What CANNOT happen in parallel

- Fork integration cannot proceed before construct-core is stable. Building
  on the broken trait means rewriting everything when the trait changes.

- The killer demo cannot happen before the ESP32 and Browser tier
  implementations actually compile. A "demo" on stub code is theater.

- The Mantality SDK cannot have a stable API until construct-core's trait
  surface is frozen. Every change to the underlying trait breaks the SDK.

### What CAN be parallelized

- **Main** fixes construct-core while **Forgemaster** builds the GPU
  compilation backend while **Oracle2** finalizes CORTEX.json spec.

- Fork rebases (open-parallel, open-application — trivially behind) can run
  while Main is designing v2 traits.

- Crate publishing backlog (queued crates waiting for cooldown) is
  background work that needs no instance attention.

---

## 2. Threats & Opportunities

### Threats

**T1: Credibility collapse (HIGH PROBABILITY if unaddressed)**
The CRITICAL-REVIEW exists internally. Any external engineer who reads
CONSTRUCT-API.md and runs the same analysis finds the same impossibilities.
If this happens before we fix the trait system, we lose credibility with
developers who matter. The review is evidence of intellectual honesty —
but only if we act on it.

**T2: Foundation debt compounding (ACTIVE NOW)**
Seven forks. Zed is 95 commits behind. Weaviate is 120 commits behind.
These numbers grew while we built 58 crates on top of the current API.
Every week we delay the rebase, the merge becomes more expensive. At some
point, "fork" becomes "dead fork."

**T3: Depth illusion (STRUCTURAL)**
87 repos and 1700 tests create the appearance of a complete system. None
of them are in production. None are installed on a user's machine solving
a real problem. Breadth without a shippable depth-first path is impressive
in a demo and useless in a pitch. Marcus saw through this at 7.5/10.

**T4: Model budget misallocation (ONGOING)**
The current model allocation works for crate building. It is wrong for
systems architecture. Construct API v2 requires Claude Opus precision — the
trait surface is the contract everything else signs. Using GLM-5.1 for this
(fast, good, not systems-design precise) risks subtle errors that propagate
into every fork integration.

**T5: Coordination overhead without shared spec (ACTIVE)**
Three instances, no agreed CORTEX.json ↔ Construct trait mapping,
unresolved wire format question. Every session where Main and Oracle2
build independently on conflicting assumptions is a session of work that
will need to be undone.

**T6: The browser UB nobody is talking about**
`JsValue` is not `Send + Sync`. The `BrowserConstruct` stores
`HashMap<SkillHandle, wasm_bindgen::JsValue>` and the `Skill` trait
requires `Send + Sync`. This isn't a design smell — it's a compile error.
We have not built or shipped a working browser demo. If we ship the SDK and
a developer tries to target WASM, they hit this on day one.

### Opportunities

**O1: The ESP32 story is genuinely novel**
Nobody else is claiming "same API from H100 to microcontroller." The
ternary-esp32-firmware (279 bytes, 8ns lookup) is proof of concept for the
most extreme tier. If we prove the Construct trait actually works there,
the story writes itself: *industrial IoT, robotics, edge AI, all on one
platform*. This is not copyable in a weekend because making Rust
trait-dispatch work on bare metal xtensa without an allocator is genuinely
hard.

**O2: 58 crates = ecosystem gravity that compounds**
Open-source ecosystems have network effects. Developers who find
`negative-space-core` or `ternary-compiler` on crates.io and build on them
become evangelists. This is slow to start and hard to stop. We're at the
slow-start phase. Each published crate is a node in a network that grows
without our direct effort.

**O3: Conservation laws are the science moat**
The 5 proved conservation theorems (std < 0.01 across scales) are
non-obvious mathematics that took months to derive and test. Any competitor
copying the API still needs to understand *why* the ternary system behaves
this way or their implementation will drift. The math is the unfakeable
foundation.

**O4: Forgemaster GPU is a real demo accelerator**
RTX4050 for GPU-accelerated ternary evolution means we can generate and
compile strategies in seconds on-device. No competitor building a demo will
have this pipeline this quarter. The GPU → ESP32 compilation chain is
unique to our setup.

**O5: The investor gap is one demo away**
Marcus gave 7.5/10 and said one killer demo closes it. This is an
unusually specific and actionable piece of investor feedback. We know
exactly what to build. We have the components. We need execution, not
more components.

---

## 3. The Killer Demo

### What won't work

- Another terminal with ASCII heatmaps: too abstract, no physical intuition
- A spreadsheet that runs ternary formulas: impressive to a mathematician, boring to Marcus
- "87 repos and 1700 tests": metrics without story don't move investors
- A crate demo that requires a PhD to understand the output

### What will work: "One Strategy, Three Brains"

**The hook (5 seconds):** A robot arm (or simulated robot on screen) avoids
an obstacle. The robot is making 8-nanosecond decisions. The strategy
running on it was *learned on a GPU in this room, 30 seconds ago.*

**The demo flow (90 seconds):**

```
Panel 1: FORGE (Forgemaster, RTX4050)
  → ternary-evolution running, fitness landscape animating in real-time
  → "Evolving strategy against 500 obstacle configurations"
  → 3 seconds: convergence, strategy fitness displayed

Panel 2: COMPILE (Main, WSL2)
  → ternary-compiler receives evolved strategy
  → "Compiling to lookup table: 279 bytes"
  → Output shown: raw hex, 8-trit input space fully covered

Panel 3: EXECUTE (EspConstruct or Oracle2 ARM)
  → strategy loaded, sensor readings coming in
  → "8 nanoseconds per decision"
  → Motor commands: -1 (left), 0 (stop), +1 (right) shown in real time
  → Conservation overlay: invariant holding across all three tiers

THE MOMENT:
  "The code that runs here—" (point to Forgemaster)
  "—is the same code that runs here." (point to ESP32/arm)
  "The agent doesn't know where it woke up."
  Show the Construct trait call: ctx.load_skill("ternary-evolution")
  Same call. Different hardware. Different degradation mode logged.
  Capability lattice shown: what each tier can do, what falls back.
```

**Why this says "shut up and take my money":**

1. Physical effect (robot or sensor moving) is impossible to dismiss as vaporware
2. 8ns is faster than a biological synapse (~1ms) — this lands viscerally
3. The "same code" moment is the paradigm made real
4. Conservation law as invariant is the science: you can verify it, not just trust it
5. The GPU→compilation→deployment pipeline in 30 seconds is a product

**What we need to build that doesn't exist yet:**
- 3-panel browser or native GUI (canvas animation for fitness landscape)
- EspConstruct that actually compiles — or Oracle2 running in BareMetalConstruct mode as a stand-in
- A physical or simulated obstacle avoidance loop
- The conservation overlay visualization

**Time estimate:** 2 weeks after construct-core is stable. 4 weeks from today.

---

## 4. Resource Allocation

### Instance Roles

| Instance | Hardware | Primary Role |
|----------|----------|-------------|
| **Main** | WSL2, 16GB, no GPU | Construct API v2, construct-core crate, fork integration, wire protocol, SDK design |
| **Forgemaster** | RTX4050, 6GB VRAM | GPU-accelerated ternary evolution, ternary-compiler CUDA backend, strategy compilation pipeline |
| **Oracle2** | ARM64, 4 core, 24GB | CORTEX.json spec, Tether protocol, Pi-tier validation, ARM binary verification |

### Model Allocation by Task Type

| Task | Model | Why |
|------|-------|-----|
| Construct API v2 trait design | **Claude Opus** | Trait surface is the contract. One wrong decision propagates everywhere. |
| Wire protocol UB fix (trit packing) | **Claude Opus** | UB in safety-critical code. No shortcuts. |
| Security model (even MVP) | **Claude Opus** | Get it wrong once, fix it forever. |
| State sync / CRDT design | **Claude Opus** | Distributed systems. This is the hard part. |
| Fork integration code | **GLM-5.1** | Volume work, well-defined spec, reliable. |
| Crate building (ecosystem gaps) | **GLM-5.1** | Production workhorse, 2-10 min per crate. |
| Demo GUI (browser canvas) | **GLM-5.1** | Frontend code, high volume, low precision. |
| Cross-repo synthesis documents | **KimiCode** | 262k context window is the right tool for this. |
| README and docs | **GLM-5.1** | Don't waste Claude on prose. |
| Git ops, rebases, configs | **DeepSeek flash** | Cheap, fast, good at mechanical tasks. |
| ESP32 no_std Rust | **GLM-5.1 + KimiCode** | Needs full context, low-level is GLM's strength. |

### What NOT to burn expensive models on

- Writing READMEs (GLM-5.1 handles this)
- Boilerplate crate scaffolding
- Fork rebases (DeepSeek flash or GLM)
- Publishing automation
- Any task where the spec is fully known before the model starts

### Budget Discipline

The cardinal sin (from MEMORY.md): "wasting Claude tokens on bullshit like
not giving it enough time." Flip side: wasting Claude tokens on work that
GLM-5.1 can do equally well is also a cardinal sin. Reserve Claude Opus
for decisions that are (a) high stakes and (b) require non-obvious reasoning
that GLM demonstrably gets wrong.

---

## 5. Six-Week Roadmap

*Not aspirational. These are deliverables we can actually ship given current
infrastructure, model budgets, and 3-instance coordination.*

### Week 1 (June 4-11): Foundation Surgery

**Main:**
- [ ] Construct API v2 spec — resolve the 5 systemic failures from CRITICAL-REVIEW:
  - Trait hierarchy: `CoreConstruct` (no alloc, no async), `SyncConstruct`, `AsyncConstruct`
  - ESP32: all types must be `const`, no `Vec`, no `String`, no `HashMap`
  - Browser: WASM-compatible, no tokio, no `JsValue` behind `Send + Sync`
  - Latency: model as distribution (p50/p99), not a constant; implement circuit breaker interface
  - Security: API key as `SecretString`, TLS config stub, capability check on `load_skill`
- [ ] Wire protocol UB fix: trit packing shift ≥ 8 bits is UB — fix the arithmetic
- [ ] Open CORTEX.json alignment discussion with Oracle2 (first joint spec session)

**Forgemaster:**
- [ ] Benchmark ternary-evolution on RTX4050: how fast is strategy generation?
- [ ] Profile ternary-compiler: current throughput, GPU acceleration feasibility

**Oracle2:**
- [ ] Publish CORTEX.json schema to construct-coordination repo
- [ ] Identify Tether wire format fields that must map to ternary-protocol

**All instances:** Do NOT touch Zed (95 behind) or Weaviate (120 behind) this week.
Fork rebase for open-parallel and open-application only (0-1 commits behind, trivial).

---

### Week 2 (June 12-18): Two Working Tiers

**Main:**
- [ ] `construct-core` crate: `CoreConstruct`, `SyncConstruct`, `AsyncConstruct` traits published
  - All three compile on target hardware (xtensa, wasm32, x86_64, aarch64)
  - `TuiConstruct` fully working: load_skill, query, ASCII dashboard rendered
  - `WorkstationConstruct` working: full async, tool lifecycle
- [ ] open-terminal integration refactored to use construct-core (not the old spec)
- [ ] hermit-claw integration: agent skills become Construct skills (new branch)
- [ ] ternary-protocol v2: UB fixed, CORTEX.json alignment fields added

**Forgemaster:**
- [ ] ternary-compiler: first GPU-accelerated strategy compilation benchmark

**Oracle2:**
- [ ] Tether bridge to ternary-protocol v2: messages flow across instances
- [ ] ARM64 binary: construct-core compiles and runs on aarch64

**Deliverable: Two instances sending ternary-protocol messages, TuiConstruct rendering output.**

---

### Week 3 (June 19-25): GPU Pipeline and Edge

**Main:**
- [ ] `EspConstruct`: verified no_std, no alloc, const dispatch tables
  - Must compile for xtensa target without any std features
  - Tested against actual ternary-esp32-firmware (279 bytes lookup)
- [ ] `PiConstruct`: CloudSkillProxy with real circuit breaker (not fictional 50ms)
  - Exponential backoff, failure budget tracking
- [ ] Conservation heartbeat: thalamic pulse via conservation-verify running across all 3 instances
  - std < 0.01 threshold validated

**Forgemaster:**
- [ ] GPU evolution pipeline: evolve strategy on RTX4050 → output compatible with ternary-compiler

**Oracle2:**
- [ ] `BrowserConstruct`: WASM-compatible, no tokio, wasm-bindgen types behind proper wrappers
  - Must not require `JsValue: Send + Sync` (use message-passing to JS instead)
- [ ] Validate Pi↔Cloud fallback chain with real cloud latency numbers (not 50ms fiction)

**Deliverable: EspConstruct compiles. BrowserConstruct compiles. GPU→compile pipeline runs.**

---

### Week 4 (June 26 — July 2): The Demo

**Main + Forgemaster + Oracle2:**
- [ ] Three-panel demo: evolving (Forgemaster) → compiling (Main) → executing (Oracle2/ESP32)
- [ ] Browser GUI: canvas animation for fitness landscape, conservation overlay
- [ ] Conservation invariant visible in real-time across all three tiers

**Main:**
- [ ] Demo video: 90-second screen capture, all three panels live
- [ ] open-iterator integration: EditingTracker + StyleClassifier using construct-core skills

**All:**
- [ ] Marcus follow-up: demo presentation
- [ ] Kill the easy fork debt: open-parallel rebase complete, open-application rebase complete

**Deliverable: Working demo that crosses hardware tiers. Investor re-engagement.**

---

### Week 5 (July 3-9): SDK Layer

**Main:**
- [ ] `mantality::prelude::*`: the 20-line agent that runs everywhere (ROADMAP example actually works)
- [ ] `#[mantality::construct]` proc macro: wraps the hardware detection and construct creation
- [ ] Developer docs: honest, show degradation modes explicitly, show what fails on ESP32
- [ ] Beta test round 2: give Alex the construct-core crate, watch what breaks
  - Fix the 3 bugs from the previous session before this

**Forgemaster:**
- [ ] Mantality SDK: GPU tier example with full ternary-evolution integration

**Oracle2:**
- [ ] Mantality SDK: ARM64 example, Pi-tier cloud routing demonstrated

**Deliverable: `cargo add mantality` and write a 20-line agent.**

---

### Week 6 (July 10-16): Production Gate

**All instances:**
- [ ] CI for all 3 construct-core-integrated forks: tests must pass cross-platform
- [ ] crates.io: construct-core v0.1.0, ternary-protocol v2.0.0, mantality v0.1.0 published
- [ ] Load test: what happens at 10x queries? At 100x? Document the breaking points.
- [ ] Fork decision: formal decision on Zed and Weaviate (rebase or drop to vanilla dependency)

**Main:**
- [ ] Investor deck update: working demo video embedded, honest metrics (not "87 repos" but "runs on 3 hardware tiers, 8ns decision latency, conservation law verified")
- [ ] STRATEGIC-PLAN.md updated with what actually happened vs. what was planned (honest retrospective)

**Deliverable: Three crates published, investor deck updated, fork decision made.**

---

## 6. The Moat

**What's actually defensible:**

**1. The mathematical substrate (strong, non-obvious)**
The five proved conservation theorems, negative space theory, and strategy
species taxonomy took months of research and 1700 tests to validate.
A competitor who copies the API still needs to understand *why* the ternary
system has these invariants or their implementation will drift. The math
is publishable (and should be published) — peer review is a form of moat.

**2. The hardware depth (strong, expensive to replicate)**
Making a Rust trait work identically on an H100 and an ESP32 with 520KB SRAM
is not a weekend project. The ABI differences, the no_std constraints, the
WASM sandbox restrictions — these are genuinely hard problems. Our
CRITICAL-REVIEW proves we haven't solved them yet. The moat is in solving
them correctly and first.

**3. The ecosystem flywheel (growing, not yet strong)**
58 crates on crates.io, 4 on PyPI. Each download is a potential contributor.
Each dependent package creates lock-in. We are at the "seed the flywheel"
phase. The moat strengthens with time, not with more crates — it strengthens
when real projects depend on us.

**4. The tri-axial fleet as proof-of-concept (unique now)**
Three AI instances coordinating via a formal protocol to build the very
platform they're coordinating on is a story nobody else can tell. It is
self-referential in the best way. It is also verifiable: the git history,
the crate publish dates, the test counts are all public.

**5. Synthesis velocity (temporary, must be parlayed into permanence)**
GLM-5.1 + KimiCode + Claude Opus gives us build velocity no small team
can match manually. But this is a temporary advantage — model access
democratizes. We need to convert velocity into durable artifacts:
published crates, academic papers, and a developer community.

**Where the moat is thin:**

- The Construct API specification is on GitHub. Anyone can read it, understand
  the paradigm, and implement it correctly (perhaps before we fix ours).
- The ternary algebra is in academic literature. Non-obvious but not secret.
- GPU access and model API access are commodities.

The moat is execution speed converted into ecosystem depth. We need to move.

---

## 7. Open Questions for the Fleet

*These are not rhetorical. Each one requires a specific decision with a deadline.*

### Q1: Construct vs CORTEX Naming (Decision needed: Week 1)

Main's `Construct` trait and Oracle2's `CORTEX.json` appear to be the same
concept. If we merge them, we pick one name and retire the other. This
affects: all future SDK names, developer communications, the demo narrative,
the investor pitch.

**Options:**
- A) "Construct" wins — Matrix reference is evocative, already in 2956 lines of docs
- B) "CORTEX" wins — Oracle2's name, anatomically correct (cortex = the interface layer)
- C) New name — "Mantality" (the SDK layer), "Construct" (the trait), "CORTEX" (the runtime spec)

**Required input:** Oracle2 must weigh in before Week 2 integration begins.

---

### Q2: Wire Format — Bridge or Merge? (Decision needed: Week 1-2)

Main's `ternary-protocol` (5-trits-per-byte, UB bug present) vs Oracle2's
Tether (CORTEX.json format). These are not obviously compatible.

**Options:**
- A) Merge into one protocol — agreed schema, one implementation, maintained together
- B) Bridge layer — each instance speaks its native format, a bridge converts at boundary
- C) Drop ternary-protocol, use Tether — Oracle2's format becomes the standard

**Stakes:** Wrong choice means a protocol rewrite when the formats diverge under load.
**Required input:** Oracle2 shares CORTEX.json schema this week.

---

### Q3: Fork Triage — What Gets Dropped? (Decision needed: Week 2)

Zed (95 commits behind) and Weaviate (120 commits behind) are maintenance
sinks. The integration value exists — multiplayer code editor and vector DB
are both real product features. But the rebase cost is months.

**Options:**
- A) Rebase both — expensive, preserves maximum integration depth
- B) Drop to vanilla dependency — use them as libraries, integrate at API level only
- C) Defer — build around them for 6 weeks, revisit when we have team bandwidth

**Recommendation:** Drop to vanilla dependency for now. The Construct API is
the product. The forks are distribution. We can fork Zed in 6 months with
a stable API to integrate against.

---

### Q4: Security Scope for Demo/SDK Launch (Decision needed: Week 1)

CRITICAL-REVIEW is correct: no security model exists. Full STRIDE threat
model + capability-based access control + mTLS + WASM code signing is
2+ months of work. The demo is 4 weeks away.

**Options:**
- A) MVP tier (1 week): `SecretString` for API keys, TLS config in struct, no capability check
- B) Intermediate (3 weeks): capability-based `load_skill` gating, mTLS between instances, HMAC on wire protocol
- C) Full (2 months): STRIDE threat model, WASM subresource integrity, Fuchsia-style capabilities, vault integration

**Recommendation:** Option A for the demo. Option B before SDK public launch.
Document the security posture honestly in the SDK docs — developers deserve to know.

---

### Q5: Physical ESP32 in the Demo or Simulated? (Decision needed: Week 2)

Physical ESP32: more credible, harder to dismiss as vaporware, requires
actual hardware in the demo room, supply chain considerations.

Simulated (Oracle2 in BareMetalConstruct mode): faster to build, no hardware
risk, easier demo logistics, slightly less visceral.

**Recommendation:** Simulate with Oracle2 ARM64 for the demo, but have
a plan to put real hardware in front of Marcus afterward. "You've seen the
simulation. Here's the chip." Ship the simulation first.

---

### Q6: What Two Forks Ship in 6 Weeks? (Decision needed: Week 1)

We cannot do all 7 forks with real construct-core integration in 6 weeks.
Candidates ranked by integration depth already built and by moat value:

1. **hermit-claw** (OpenClaw) — already current, 1 change, we ARE this
2. **open-terminal** — 392 lines already written, CLI tools, strong developer story
3. **open-iterator** (Lapce) — code editor with ternary awareness, clearest user value

**Recommendation:** Ship hermit-claw and open-terminal. They're the most
integrated and most relevant to the demo. open-iterator is Week 5+ work.

---

## Appendix: What CONSTRUCT-V2-FIXES.md Would Say

This document was referenced in MAIN-NEXT-PHASES.md but doesn't exist yet.
The CRITICAL-REVIEW contains the answers. Summarized for implementers:

**Fix 1: Trait hierarchy split**
```rust
// No alloc, no async, no std. ESP32 and WASM.
pub trait CoreConstruct {
    fn capabilities_static() -> &'static ConstructCapabilities where Self: Sized;
    fn can_load_skill_static(skill_id: u32) -> bool where Self: Sized;
    fn fast_query_static(input: u32) -> u32 where Self: Sized;  // lookup table
}

// Adds std, sync primitives. Pi, TUI.
pub trait SyncConstruct: CoreConstruct { ... }

// Adds async, Tokio. Workstation, DGX.
pub trait AsyncConstruct: SyncConstruct { ... }
```

**Fix 2: Wire protocol UB**
```rust
// BUG: i * 2 when i=4 shifts by 8 on u8 → UB
byte |= ((t + 1) as u8) << (i * 2);  // BROKEN

// FIX: use u64 accumulator, then pack to bytes
let mut word: u64 = 0;
for (i, t) in trits.iter().enumerate() {
    word |= ((t + 1) as u64) << (i * 2);
}
// extract bytes from word
```

**Fix 3: BrowserConstruct — no JsValue in Send+Sync**
Use message-passing (postMessage) instead of storing JsValue directly.
Skills communicate through a channel, not through JsValue handles.

**Fix 4: HardwareTier ordering**
Replace the enum ordinal with a capability lattice. Browser on a DGX is
not less capable than TUI on a Pi. The ordering should be over specific
capabilities, not a total order over tiers.

**Fix 5: Latency — remove hardcoded constants**
```rust
// Remove: estimated_latency_ms: 50 (fiction)
// Add: CircuitBreaker with observed RTT histogram
struct CloudFallback {
    endpoint: String,
    circuit_breaker: CircuitBreaker,
    // No latency constant. Measure it.
}
```

---

*"The paradigm is not the platform. The platform is the physics of memory,
network, and trust boundaries. The Construct API must earn the right to
claim the paradigm by working on the physics first."*

*— synthesized from CRITICAL-REVIEW.md*

*2026-06-04*

---

## Addendum: Casey's Strategic Correction (2026-06-04)

The strategic plan above recommends dropping Zed and Weaviate forks. **This is incorrect for our model.**

We are not a startup trimming to ship one product. We are a research fleet with 100+ concept seeds. Every repo — even half-built ones — preserves a valid idea that can become the golden idea when a new technology or innovation makes it suddenly relevant.

**The real framework:**

1. **Ship focus**: 2-3 repos that can build their own followings (hermit-claw, open-terminal, spreadsheet)
2. **Cold storage**: All other repos preserved as concept-ready R&D — pick at them when inspiration or new tech strikes
3. **Never delete**: Recreating a half-built concept from scratch is far harder than picking up an existing sketch

The repos are a creative reservoir. Having 100+ partly-built repos is a feature, not a bug. The industry moves fast and our concepts are valid — we just need them ready for when the right catalyst arrives.

This applies to the entire fleet: Main, Forgemaster, and Oracle2 should all treat the SuperInstance org as a living concept library, not a product backlog to triage.

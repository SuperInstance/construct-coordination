# Creative R&D — Breadth-First Exploration

**Date:** 2026-06-15 03:50 UTC  
**Context:** Oracle2, 4-core ARM64, 23GB RAM, 19GB free, THP active, headspace-rs multi-threaded + coalescing, γ~720-800, η=390 hard

Round 3 findings give us a rich set of next moves. But before committing to the "best" one, let me explore unconventional directions. Creativity is about breadth before depth.

## 9 Unconventional R&D Directions

### 1. 🧬 Self-modifying C code — Evolutionary optimization
Not Rust. Not Python. Compile C with `-O3 -march=native -fprofile-generate`, run PGO training, then use `-fprofile-use` and `-fauto-profile`. The 73-segment vector store loop is small enough that autovectorization + profile-guided branch prediction could get 2-3× on the hot path. Then: **genetic algorithm** mutates compiler flags (loop unrolling factors, prefetch distance, function splitting) and measures empirical γ delta. Survival of the fittest compiler flags.

### 2. 🔮 Predictive gamma scheduling — M5 forecasting
γ oscillates 704-993 with an apparent period. Use an M5 state-space model (or even a simple ARIMA) on the 30-sample γ time series to predict when γ will spike, then proactively shed load or throttle non-critical work *before* the spike hits. Turn a reactive meter into a predictive governor.

### 3. 📦 WASM-in-VM — ZeroClaw as production isolate
Run *headspace-rs itself* inside a ZeroClaw sandbox (landlock + bwrap). The hypothesis: the sandbox's memory isolation could force better cache behavior by eliminating background process noise on core 3. If headspace-rs gets the entire core to itself with no kernel scheduler interference from the other 14 services, η might finally budge.

### 4. 🧵 Preemptible generation counter
Instead of arc_swap (T8's finding) or async RwLock, use a **monotonically increasing generation counter** on the vector store. Readers do: 
```
let gen = GEN.load(Relaxed);
let result = unsafe { read_without_lock() };
if gen == GEN.load(Relaxed) { return result; } // data consistent
```
No atomics on the hot path except the reload check. Optimistic lock elision in software. The 73-segment store is so small that writes finish before a reader finishes scanning — the generation check always passes on concurrent reads. This is *faster* than arc_swap.

### 5. 🎯 Speculative execution — Prefetch with intent
The 7 nearest-neighbor queries (after T5's binary tree) do 7×384-dim dot products. The Neoverse-N1 has a `PRFM` prefetch instruction. If we prefetch the *next* segment's embedding vector while computing the *current* one, we hide L2 cache latency. Rust's `core::arch::aarch64::_prefetch` makes this explicit. Expected: ~10-20% on the vector search hot path.

### 6. 🌀 Stratified sampling — Don't search all segments
The 73 segments have different sizes (some 100 chars, some 10KB). Instead of always searching all 73 (or even 7 with binary tree), use **stratified sampling**: pre-cluster segments into ~5 buckets by content type, search the closest bucket first, and early-exit if confidence > threshold. If the query embedding is close to anything in bucket A (metrics), don't search bucket E (templates).

### 7. 🏓 Cache-aware segment layout — Hot/cold splitting
The vector store's segment order is insertion-time, not access-time. Reorder segments by access frequency: the 10 most-queried segments get pinned in L1d cache via `mlock` + aligned allocation. The remaining 63 sit in L2. The linear scan hits the hot 10 first (L1, ~2 cycles) before the cold 63 (L2, ~20 cycles). This is a *data layout* optimization, not a code optimization.

### 8. ⚡ Request fusion — Embed-and-search co-location
The current pipeline: (a) receive text → (b) compute embedding → (c) search store → (d) respond. Steps (b) and (c) use the same embedding vector. If they're fused into one operation — compute embedding AND search simultaneously — the embedding vector stays in registers between steps. No write-back to L1d, no read-back from L1d. Expected: ~5-10% on the per-query path.

### 9. 🌊 Token streaming — Partial prefix matching
Instead of waiting for a full query to arrive, start matching on token prefixes *as the query is being received*. If the first 10 chars match a known segment prefix, begin the nearest-neighbor search with a narrowed candidate set. By the time the full query arrives, the search is 50% done. This is speculative computation — if the prefix is wrong, discard and restart. The latency improvement for repeated queries (which most are) is ~40%.

---

## Selection Criteria

| Idea | Risk | Impact | Novelty | Quick Win? |
|------|------|--------|---------|------------|
| 1. Genetic compiler flags | Medium | 2-3× on hot path | 🚀 | 🔄 (overnight) |
| 2. Predictive γ scheduling | Low | 10-15% | 🚀 | ✅ |
| 3. WASM-in-VM isolation | High | Uncertain | 💫 | ❌ |
| 4. Preemptible gen counter | Low | ~50% η reduction | 💫 | ✅ (code change) |
| 5. PRFM prefetch | Low | 10-20% | 🛠️ | ✅ (code change) |
| 6. Stratified sampling | Low | ~50% γ reduction | 🚀 | ✅ (code change) |
| 7. Hot/cold segment layout | Medium | 20-30% | 🛠️ | ✅ (one-time reorder) |
| 8. Embed-search fusion | Medium | 5-10% | 💫 | 🔄 (refactor) |
| 9. Token streaming | High | ~40% | 🚀 | ❌ |

**Recommendation:** Start with the 4 quick wins (2, 4, 5, 6) in parallel — they're low risk, modest code changes, and their effects are orthogonal. The genetic compiler flags (1) runs in the background. The high-risk ideas (3, 9) are exploratory prototypes.

---

*Breadth-first exploration for Casey's R&D push*

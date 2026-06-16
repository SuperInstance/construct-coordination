# Pincher Audit — 2026-06-15

## Verdict: ❌ NOT PRODUCTION-READY — Critical CI/Feature Issues Remain

---

## 1. `cargo check` & `cargo test` Results

| Check | Status | Details |
|-------|--------|---------|
| `cargo check` | ✅ Pass | Default features only |
| `cargo test` (all) | ✅ Pass | 321 tests passed, 7 doc-test ignored, 0 failures |
| `cargo clippy -- -D warnings` | ✅ Pass | No clippy warnings |
| `cargo build --features onnx` | ❌ **FAIL** | 4 compilation errors — ORT API broken |

**Breakdown by crate (default features):**

| Crate | Tests | Status |
|-------|-------|--------|
| `hybrid-bridge` lib | 104 | ✅ Pass |
| `pincher-core` lib | 156 | ✅ Pass |
| `pincher` bin | 2 | ✅ Pass |
| CLI integration tests | 18 | ✅ Pass |
| Full pipeline tests | 1 | ✅ Pass |
| Core integration tests | 16 | ✅ Pass |
| Doc-tests | 8 total (1 pass, 7 ignored) | ✅ Skip |

---

## 2. Dependency Tree Analysis (`cargo tree --depth 3`)

### Flagged Outdated Major Versions

| Dependency | Current | Latest (major) | Risk |
|-----------|---------|----------------|------|
| `thiserror` | 1.0.69 | **2.x** available since Nov 2024 | v2 is the new standard; **v1 deprecated by ecosystem** — will eventually break |
| `ndarray` (hybrid-bridge) | 0.16.1 | **0.17.x** | v0.16 still maintained but v0.17 has better integration |
| `arrow` family | 53.4.1 | 54.x+ | Arrow releases monthly — 53 is already behind; API drift risk |
| `rusqlite` | 0.31.0 | 0.32.x | v0.32 adds better `bundled` support, v0.33 in preview |
| `serde_json` | 1.0.150 | 1.0.x is latest | ✅ Fine (no major version bump planned) |
| `sqlite-vec` | 0.1.9 | pre-1.0 | ✅ Fine — early but stable |

### ⚠️ Duplicate Crates (bloat + semver risk)

| Crate | Versions | Impact |
|-------|----------|--------|
| **`ternary-types`** | **v0.1.0 + v0.2.0** | **HIGH** — Two different git revs of the same lib compiled in the same workspace. `pincher-core` pins `v0.1.0` (via Cargo.lock), `hybrid-bridge` pins `v0.2.0` (workspace dep). These are DIFFERENT code — risk of type mismatch across crate boundaries. |
| `bitflags` | v1.3.2 + v2.12.1 | LOW — v1 is transitive from `flatbuffers`, expected |
| `getrandom` | v0.3.4 + v0.4.2 | LOW — v0.3 from `ahash` v0.8, v0.4 from `uuid` v1.23; expected |
| `hashbrown` | v0.14.5 / v0.15.5 / v0.17.1 | LOW — Expected from multiple transitive paths |
| **`ndarray`** | **v0.16.1** (hybrid-bridge) vs **v0.17** (pincher-core, optional) | MEDIUM — pincher-core was bumped to 0.17 in the CI fix commit but hybrid-bridge still uses 0.16. Two versions in the workspace. Same ndarray in two versions can cause feature conflicts. |

### Git Dependencies (non-crates.io)

| Dependency | Source | Risk |
|-----------|--------|------|
| `ternary-types` | GitHub, pinned rev | **HIGH** — Two different revs. One in workspace (v0.2.0@fa01da44), one in `pincher-core` (v0.1.0@1dae82f8). If those revs diverge, builds break. |
| `silo-core` | GitHub, pinned rev | MEDIUM — Workspace dep declared but **never used** in any Cargo.toml dependency list. Dead code / unused workspace dependency. |

---

## 3. CI Config Analysis

### File: `.github/workflows/ci.yml`

**✅ CI is correct for default feature builds.** Steps:
1. `actions/checkout@v4` — latest major ✅
2. `dtolnay/rust-toolchain@stable` with clippy + rustfmt ✅
3. `Swatinem/rust-cache@v2` with `cache-on-failure: true` ✅
4. `cargo build --all-targets` ✅
5. `cargo test` ✅
6. `cargo clippy -- -D warnings` ✅
7. `cargo fmt --check` ✅

**❌ ISSUES:**

1. **Does NOT test `--features onnx` or `--features landlock`** — The `onnx` feature is completely broken (see §1), and this would be caught immediately by a feature matrix build. Add a `matrix.features` job.

2. **Does NOT run `bwrap` or `landlock` integration tests** — Tests like `test_sandbox_sandbox_capability_manifest` show `bwrap: Can't find source path /lib64` warnings in local runs. CI uses `ubuntu-latest` which also may not have `bwrap` preinstalled.

3. **No artifact caching across `cargo test` and `cargo clippy`** — `cargo clippy` re-checks the entire crate. Use `--all-targets` on the clippy step or cache results.

4. **No `cargo deny` / `cargo audit`** — No supply chain security checks in CI.

### File: `.github/workflows/publish_nail.yml`

**RISKS:**
- Requires `DEEPINFRA_API_KEY` and `PINCHER_SIGNING_KEY` secrets — not documented in any README or CONTRIBUTING
- `pincher compile` and `pincher mature` are custom commands not tested in CI's `cargo build --all-targets`
- Hardcoded `https://pincher.dev` registry URL that doesn't exist in CI context

### File: `.github/workflows/release.yml`

- Missing `verify` or `sign` step on the published binary
- No multi-arch support (only `ubuntu-latest` / x86_64)

---

## 4. Remaining Risks (Production Readiness)

### Critical 🛑

| # | Risk | Details |
|---|------|---------|
| **R1** | **`--features onnx` does not compile** | 4 errors from `ort 2.0.0-rc.12` API drift beyond what was fixed. `ort::prelude` was removed, `session.run()` returns a different type, `Value::shape()` returns a tuple not a shape ref. **All ORT-dependent functionality is dead code.** |
| **R2** | **`ternary-types` version conflict** | Two different git revisions (`v0.1.0` vs `v0.2.0`). If binary size or type compatibility across crate boundary is needed (e.g., serializing/deserializing ternary values between crates), this will silently produce wrong results or fail at runtime. |

### High 🔴

| # | Risk | Details |
|---|------|---------|
| **R3** | **CI doesn't test feature-gated code** | `onnx`, `landlock`, `wasmtime`, `ternary-kernel` features are completely untested in CI. They could be broken at any time without detection. |
| **R4** | **No supply chain auditing** | No `cargo-deny` or `cargo-audit` in CI. If a transitive dep has a CVE, you won't know until it hits prod. |
| **R5** | **`silo-core` dead workspace dependency** | Declared in workspace deps but zero uses. Adds confusion and potential for versioning issues. |

### Medium 🟡

| # | Risk | Details |
|---|------|---------|
| **R6** | **`ndarray 0.16` / `0.17` split** | hybrid-bridge uses 0.16, pincher-core (optional) uses 0.17. If ndarray types cross crate boundaries, they won't be compatible. |
| **R7** | **`thiserror v1` nearing EOL** | Ecosystem is moving to `thiserror v2` (stable since Dec 2024). v1 is in maintainence-only. Migration needed, especially for derive macros that changed in v2. |
| **R8** | **Arrow 53.x is 2-3 months behind** | Arrow releases monthly; v53 was Nov 2025. Feature additions and bug fixes in v54+. Locking to a minor is fine but should track latest patch at minimum. |
| **R9** | **No `arm64` CI runner** | Repository targets aarch64 (Ampere), but CI runs on `ubuntu-latest` (x86_64). NEON SIMD kernel and any aarch64-specific code is untested in CI. |
| **R10** | **`bwrap` integration test warnings** | `test_sandbox_*` tests print bwrap errors in CI-like environments. Tests *pass* but with noise that could mask real failures. |

### Low 🟢

- `orth` (typo of `ort`?) is mentioned in older docs but not in code
- `fastrand` dep in hybrid-bridge with no usage found in current code
- `colored` dep in pincher-cli is at v2.2.0 (current)

---

## 5. Recommended Action Items

| Priority | Action | Est. Effort |
|----------|--------|-------------|
| **P0** | Fix `onnx` feature: adapt to `ort 2.0.0-rc.12` final API | 2-4h |
| **P0** | Add `cargo build --features onnx,landlock,wasmtime` to CI matrix | 30min |
| **P1** | Unify `ternary-types` to a single version (use workspace dep `v0.2.0` for both crates) | 1h |
| **P1** | Add `cargo-deny` or `cargo-audit` to CI | 30min |
| **P2** | Remove `silo-core` dead workspace dependency | 5min |
| **P2** | Bump `thiserror` to v2 across workspace | 1h |
| **P2** | Bump `ndarray` in hybrid-bridge to 0.17 to match pincher-core | 30min |
| **P3** | Add aarch64 CI runner (e.g., `buildjet-2vcpu-ubuntu-2204-arm`) for NEON SIMD coverage | 1h |
| **P3** | Remove unused `fastrand` from hybrid-bridge deps if confirmed unused | 15min |
| **P3** | Add `cargo fmt --check` for publish/release workflows | 15min |
| **P4** | Document required secrets (`DEEPINFRA_API_KEY`, `PINCHER_SIGNING_KEY`) in CONTRIBUTING.md | 30min |

---

## 6. Summary

**Pincher is NOT production-ready.** While the default-features build and test suite passes cleanly (321 tests ✅), the `--features onnx` build is completely broken due to unaddressed `ort 2.0.0-rc.12` API drift. This means the ONNX embedding pipeline (the primary embedding mechanism) is non-functional when enabled.

Additional structural issues — duplicate `ternary-types` versions, untested feature-gated code, dead workspace deps — mean the quality bar isn't met for production deployment without significant remediation work.

**Estimated remaining work for production readiness: 6-10 hours across P0-P2 items.**

### Bottle Metadata
- **Date:** 2026-06-15
- **Host:** Oracle Ampere aarch64
- **Rust:** stable (rust-toolchain: stable)
- **HEAD:** 9fc2e49 — fix: pin ndarray 0.17 and adapt ort 2.0.0-rc.12 API
- **Branch:** main
- **Audit Depth:** cargo check, cargo test (all), cargo clippy, cargo tree --depth 3, cargo tree --duplicate, CI config review

# Ternary Crates Publishing Audit — 2026-06-16

**Scope:** `ternary-search`, `ternary-route`, `ternary-pid`, `ternary-search-rs`  
**Task:** Tier 2 — "Publish top ternary crates to crates.io — 50 crates by August"  
**Audit date:** 2026-06-16

---

## 🏷️ Crate 1: `ternary-search`

### Current State

| Field | Value | Status |
|-------|-------|--------|
| version | `0.2.0` | ✅ semver OK |
| license | `MIT` | ✅ |
| description | `"Search algorithms over ternary strategy spaces"` | ✅ |
| repository | `https://github.com/SuperInstance/ternary-search` | ✅ |
| homepage | _(none)_ | ⚠️ add |
| categories | `["algorithms", "data-structures"]` | ✅ |
| keywords | `["ternary", "search", "strategy", "beam-search", "astar"]` | ✅ |
| edition | `2021` | ✅ |
| `cargo check` | ✅ compiles clean | ✅ |

### 🚫 Blocking Issue

```
[dependencies]
ternary-types = { git = "https://github.com/SuperInstance/ternary-types.git" }
```

**`ternary-types` is a git-only dependency.** It is NOT published on crates.io. `ternary-search` cannot be published until `ternary-types` is published first.

**Dependency chain:** `ternary-search` → `ternary-types` (git, unpublished)

### Fix Path

1. Publish `ternary-types` to crates.io first (see below)
2. Change `Cargo.toml` dependency to:
   ```toml
   ternary-types = "0.1"   # once published
   ```
3. Bump `ternary-search` version to `0.3.0` (minor: new published dep)
4. Add `homepage = "https://superinstance.ai"` (optional but recommended)
5. Run `cargo update` to regenerate `Cargo.lock`
6. `cargo publish --dry-run` to verify, then `cargo publish`

### READMEs
- `README.md`: 151 lines — well-structured, includes algorithm table, examples
- `ARCHITECTURE.md`, `GETTING_STARTED.md`, `CONTRIBUTING.md`, `PLUG_AND_PLAY.md` all present
- Long-form docs are a strength for this crate

---

## 🏷️ Crate 2: `ternary-route`

### Current State

| Field | Value | Status |
|-------|-------|--------|
| version | `0.1.0` | ✅ semver OK |
| license | `MIT` | ✅ |
| description | `"Ternary routing with load balancing and failover"` | ✅ |
| repository | _(none)_ | 🚫 missing |
| homepage | _(none)_ | ⚠️ add |
| categories | _(none)_ | 🚫 missing |
| keywords | _(none)_ | 🚫 missing |
| edition | `2021` | ✅ |
| `cargo check` | ⚠️ 2 warnings | ⚠️ fix warnings first |

### 🚫 Blocking Issues

1. **Missing `repository` field** — required for crates.io publish
2. **Missing `categories`** — required for crates.io (must select from pre-defined list)
3. **Missing `keywords`** — recommended
4. **`cargo check` warnings:**
   - `unused import: HashMap` in `src/lib.rs:3`
   - `unnecessary parentheses` in `src/lib.rs:68`

### Fix Path

```toml
[package]
name = "ternary-route"
version = "0.1.0"
edition = "2021"
license = "MIT"
description = "Ternary routing with load balancing and failover"
repository = "https://github.com/SuperInstance/ternary-route"
homepage = "https://superinstance.ai"
categories = ["algorithms", "network-programming"]
keywords = ["ternary", "routing", "load-balancing", "failover", "health-check"]
```

Then:
```bash
cd /tmp/ternary-route
cargo fix --lib -p ternary-route  # auto-fix warnings
cargo publish --dry-run            # verify
cargo publish                      # publish
```

### READMEs
- `README.md`: 134 lines — good structure, explains cascading route selection with table
- `CONTRIBUTING.md` present
- `docs/` directory present

---

## 🏷️ Crate 3: `ternary-pid` (bonus — found in org)

### Current State

| Field | Value | Status |
|-------|-------|--------|
| version | `0.1.0` | ✅ semver OK |
| license | `MIT` | ✅ |
| description | `"Ternary PID controller with anti-windup and derivative filtering for fleet governance"` | ✅ |
| repository | `https://github.com/SuperInstance/ternary-pid` | ✅ |
| homepage | `https://superinstance.ai` | ✅ |
| documentation | `https://docs.rs/ternary-pid` | ⚠️ broken until published |
| categories | `["algorithms", "science", "simulation"]` | ✅ |
| keywords | `["pid", "ternary", "fleet", "control", "conservation"]` | ✅ |
| edition | `2021` | ✅ |
| `cargo check` | ✅ compiles clean | ✅ |

### ⚠️ Pre-publish Fixes

1. **`documentation` field** points to `docs.rs/ternary-pid` which will 404 until first publish. Remove it or accept the initial 404.
2. **Version bump recommended** before publish: `0.1.0 → 0.1.1` (in case patch needed post-publish)

### Fix Path

```toml
# Remove or comment out documentation until first publish succeeds:
# documentation = "https://docs.rs/ternary-pid"
```

Then: `cargo publish`

**This crate is the closest to ready — lowest effort.**

---

## 🏷️ Crate 4: `ternary-search-rs` (bonus — found in org)

### Current State

| Field | Value | Status |
|-------|-------|--------|
| version | `0.1.0` | ✅ semver OK |
| license | _(none)_ | 🚫 missing |
| description | `"High-performance ternary vector search server"` | ✅ |
| repository | _(none)_ | 🚫 missing |
| categories | _(none)_ | 🚫 missing |
| keywords | _(none)_ | 🚫 missing |
| edition | `2021` | ✅ |
| type | **Binary crate** (`[[bin]]`) | ⚠️ unusual for library ecosystem |

### 🚫 Blocking Issues

1. **No `license` field** — required
2. **No `repository` field** — required
3. **Binary crate** — crates.io is primarily for libraries; binaries are publishable but less reusable
4. **`cargo check` warnings:** dead code (`Stats` struct, `readme_length` field)

### Verdict

**Lower priority.** Binary crate with significant metadata gaps. Not a library. If the intent is to publish a library wrapper too, extract the core search logic into a library crate first.

---

## 📋 Master Fix List — Before Any Publish

### `ternary-types` (must publish FIRST)

```toml
[package]
name = "ternary-types"
version = "0.1.0"
edition = "2021"
license = "MIT"
description = "Core types for the SuperInstance ternary {-1, 0, +1} ecosystem"
repository = "https://github.com/SuperInstance/ternary-types"
homepage = "https://superinstance.ai"
categories = ["algorithms", "data-structures"]
keywords = ["ternary", "types", "-1", "0", "+1"]
```

> ⚠️ **Audit `ternary-types` itself** before publishing. Clone it and verify it has no further git-only dependencies. Also check if `ternary-types` itself depends on anything from crates.io that might be problematic.

### `ternary-search` (after `ternary-types`)

1. Change git dep to `ternary-types = "0.1"`
2. Add `homepage`
3. Bump version to `0.3.0`

### `ternary-route` (independent)

1. Add `repository`, `homepage`, `categories`, `keywords`
2. Run `cargo fix --lib`
3. Bump version to `0.1.1`

### `ternary-pid` (already nearly ready)

1. Remove or comment `documentation` field
2. Bump to `0.1.1`

---

## 🔧 Recommended `cargo publish` Workflow

```bash
# 1. Dry-run first (always)
cargo publish --dry-run

# 2. If dry-run passes, publish for real
cargo publish

# 3. Verify the crate landed
curl -s https://crates.io/api/v1/crates/<CRATE_NAME> | python3 -c "import sys,json; d=json.load(sys.stdin); print('Version:', d['crate']['latest_version'])"

# 4. Tag the release
git tag -a v0.x.0 -m "Publish to crates.io"
git push origin v0.x.0
```

**For a batch publish script** (after all fixes are applied):

```bash
#!/usr/bin/env bash
set -e

publish_crate() {
  local dir=$1
  local crate=$2
  echo "📦 Publishing $crate..."
  cd "$dir"
  cargo publish --dry-run && cargo publish
  echo "✅ $crate published"
}

# Order matters — dependents first
publish_crate /tmp/ternary-types  ternary-types
publish_crate /tmp/ternary-pid    ternary-pid
publish_crate /tmp/ternary-route  ternary-route
publish_crate /tmp/ternary-search ternary-search

echo "🎉 All 4 ternary crates published!"
```

---

## 📊 This Week Estimate

| Crate | Ready Now? | Fixes Needed | Publishable This Week? |
|-------|-----------|--------------|------------------------|
| `ternary-types` | ❌ git-only dep | Must audit + publish first | ⚠️ Yes, if clean |
| `ternary-pid` | ✅⚠️ | Remove `documentation` field | ✅ **Yes — lowest effort** |
| `ternary-route` | ⚠️ | Add 4 metadata fields + fix 2 warnings | ✅ **Yes — ~30 min work** |
| `ternary-search` | ❌ blocked | Blocked on `ternary-types` publish | ⏳ After `ternary-types` |
| `ternary-search-rs` | ❌ | Missing license/repo, binary crate | ❌ Not a library crate |

**Realistic this week:** **2 crates** (`ternary-pid`, `ternary-route`)  
**If `ternary-types` is clean:** **3 crates** (adds `ternary-search`)  
**Against the 50-by-August goal:** These 4 ternary library crates are identified. The remaining gap to 50 requires finding/creating more publishable crates across the fleet.

---

## 🔍 Next Steps (Priority Order)

1. **[HIGH] Audit `ternary-types`** — clone, check deps, publish to crates.io
2. **[HIGH] Fix + publish `ternary-pid`** — remove docs.rs link, publish
3. **[HIGH] Fix + publish `ternary-route`** — add metadata, cargo fix, publish
4. **[MED] Update `ternary-search`** — swap git dep for crates.io dep, publish
5. **[LOW] Decide on `ternary-search-rs`** — library wrapper or archive
6. **[ONGOING] Find more crates** — scan SuperInstance org for additional publishable libraries

---

*Audit by subagent · 2026-06-16 · model: minimax/MiniMax-M2.7*

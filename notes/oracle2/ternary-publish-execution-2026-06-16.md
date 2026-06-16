# Ternary Crates Pipeline: Publish Audit — 2026-06-16

## 1. Token Status

**BLOCKED — NO TOKEN FOUND**

- `~/.cargo/credentials`: does not exist
- `$CARGO_REGISTRY_TOKEN` env var: not set
- Secrets dir (`workspace/secrets/`): no cargo-related secrets found
  - `deepseek.env` → DEEPSEEK_API_KEY only
  - `fleet-kt-secret.env` → KT_FLEET_SECRET only

`cargo login` requires a token. Publishing cannot proceed without one.

---

## 2. Crate Audits

### ternary-scheduler (from GitHub `SuperInstance/ternary-scheduler`)

| Field | Value | Status |
|-------|-------|--------|
| `name` | `ternary-scheduler` | ✅ |
| `version` | `0.1.0` | ✅ |
| `edition` | `2024` | ✅ (Rust 1.96) |
| `repository` | — | ❌ MISSING |
| `homepage` | — | ❌ MISSING |
| `license` | — | ❌ MISSING |
| `description` | — | ❌ MISSING |
| `categories` | — | ❌ MISSING |
| `keywords` | — | ❌ MISSING |
| `[dependencies]` | empty | ✅ |

- **`cargo build`**: ✅ Compiles cleanly (`Finished dev profile`)
- **Verdict**: Needs all metadata fields added before publish

---

### ternary-vector

| Field | Value | Status |
|-------|-------|--------|
| Repo URL | `https://github.com/SuperInstance/ternary-vector` | ❌ NOT FOUND |

- **Verdict**: Repository does not exist (404). Crate not available for audit.

---

### Already-auditied local copies (context)

#### ternary-types (`/tmp/ternary-types`)
| Field | Status |
|-------|--------|
| `repository` | ✅ |
| `homepage` | ✅ |
| `license` | ✅ |
| `description` | ✅ |
| `categories` | ✅ |
| `keywords` | ✅ |
| `cargo build` | ✅ |

**Ready to publish as-is.**

#### ternary-search (`/tmp/ternary-search`)
| Field | Status |
|-------|--------|
| All metadata | ✅ |
| `cargo build` | ✅ |
| `[dependencies]` | `ternary-types = { git = "..." }` — **blocks crates.io publish** |

**Needs patch** (see §3).

#### ternary-route (`/tmp/ternary-route`)
| Field | Status |
|-------|--------|
| `repository` | ✅ |
| `homepage` | ✅ |
| `license` | ✅ |
| `categories` | ✅ |
| `keywords` | ✅ |
| `description` | — | **❌ MISSING** |
| `[dependencies]` | empty | ✅ |

**Needs `description` added before publish.**

---

## 3. Exact `cargo publish` Command Chain

Dependency order: **ternary-types → ternary-search → ternary-route**

### Step 1 — Publish ternary-types (no patch needed)
```bash
cd /tmp/ternary-types && cargo publish
```

### Step 2 — Patch + publish ternary-search
Before publishing, edit `Cargo.toml` to replace the git dep with the published version:
```toml
# BEFORE (blocks crates.io):
ternary-types = { git = "https://github.com/SuperInstance/ternary-types.git" }

# AFTER:
ternary-types = "0.2.0"
```
Then:
```bash
cd /tmp/ternary-search && cargo publish
```

### Step 3 — Patch + publish ternary-route
Before publishing, add a description to `Cargo.toml`:
```toml
# Add this line under [package]:
description = "Ternary routing with load balancing and failover"
```
Then:
```bash
cd /tmp/ternary-route && cargo publish
```

---

## 4. Status

```
NEEDS_TOKEN
```

Publishing is blocked. A crates.io API token must be obtained and stored in `~/.cargo/credentials` (or via `cargo login`) before any `cargo publish` command can succeed.

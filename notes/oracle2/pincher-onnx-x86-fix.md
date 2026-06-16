# Bottle: pincher-onnx-x86-fix

**Date:** 2026-06-15  
**Objective:** Fix `pincher --features onnx` compilation for ort 2.0.0-rc.12 API drift  
**Context:** ARM64 CI was broken due to ORT prebuilt binary unavailability on ARM64 and ort 2.0.0-rc.12 API changes. Fixed on x86_64 directly (no stunt-double needed since we had x86_64 env access).

## API Migration: ort → ort 2.0.0-rc.12

### 1. `ort::prelude` removed
- **Error:** `unresolved import ort::prelude`
- **Fix:** Remove `use ort::prelude::*`. Add `use std::sync::Mutex` instead (needed for thread-safe session access).
- The ort crate no longer exports a `prelude` module. Import specific types directly.

### 2. `inputs!` macro no longer returns `Result`
- **Error:** `?` operator cannot be applied to `[SessionInputValue<'_>; 3]`
- **Fix:** Remove trailing `?` from `ort::inputs![...]?`
- In ort 2.0.0-rc.12, `inputs!` produces `[SessionInputValue; N]` directly (not `Result<[SessionInputValue; N]>`), so `?` is invalid.

### 3. `try_extract_tensor` returns tuple, not typed tensor
- **Error:** `no method named 'shape' found for tuple (&ort::value::Shape, &[f32])`
- **Fix:** Destructure as `let (shape, data) = outputs[0].try_extract_tensor::<f32>()?;` instead of `let output = ...`
- `try_extract_tensor()` now returns `Result<(&Shape, &[T])>` — a tuple of shape and data slice.

### 4. Index into flat data, not ndarray indexing
- **Error:** `(&Shape, &[f32])` cannot be indexed by `[usize; 3]`
- **Fix:** Use flat index: `data[t * EMBEDDING_DIM + d]` instead of `output[[0, t, d]]`
- Since `data` is `&[f32]`, use stride-based indexing. Batch dim is 1.

### 5. `Session::run()` requires `&mut self` (new in 2.0.0-rc.12)
- **Fix:** Wrap `Session` in `Arc<Mutex<Session>>` instead of `Arc<Session>`
- Changed `EmbedderState::Loaded(Arc<Session>)` → `EmbedderState::Loaded(Arc<Mutex<Session>>)`
- Changed `embed_onnx(&self, session: &Session, ...)` → `embed_onnx(&self, session: &mut Session, ...)`
- Lock mutex at call site in `embed()`.

### 6. `Shape` elements are `i64`, not `usize`
- **Fix:** Cast with `shape[2] as usize` when comparing to `EMBEDDING_DIM` (which is `usize`).

### 7. Move semantics on `attention_mask`
- **Fix:** `.clone()` the attention_mask vec before moving into `Array2::from_shape_vec()`
- A pre-existing issue exposed by the other fixes — the `attention_mask` was consumed by `from_shape_vec` but later used for pooling weights.

## Files Changed

- `pincher-core/src/embed/onnx.rs` — all fixes in one file

## Verification

```
$ cargo check --features onnx
    Checking pincher-core v0.1.0
    Checking hybrid-bridge v0.1.0
    Checking pincher-cli v0.1.0
    Finished `dev` profile [unoptimized + debuginfo]
```

Zero errors, zero warnings.

## Key Types (ort 2.0.0-rc.12 reference)

| Type | Description |
|------|-------------|
| `Session` | ONNX Runtime session. `run()` takes `&mut self` now. |
| `SessionInputValue` | Input value wrapper. Use `ort::inputs!` macro. |
| `DynValue` | `Value<DynValueTypeMarker>` — a dynamically-typed value. |
| `Tensor<T>` | `Value<TensorValueType<T>>` — a statically-typed tensor. |
| `(&Shape, &[T])` | Returned by `try_extract_tensor::<T>()`. `Shape` derefs to `[i64]`. |
| `SessionOutputs` | Output collection. Indexable by `usize` or `&str`. |
| `Error<R = ()>` | Error type; still takes a recovery type param. |

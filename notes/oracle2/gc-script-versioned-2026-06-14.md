# 🧪 Bottle: gc-intelligent.sh versioned into construct repo

**Date:** 2026-06-14 (UTC)
**Source:** Subagent task

## What was done

1. **Copied** `workspace/scripts/gc-intelligent.sh` → `construct/scripts/gc-intelligent.sh` (chmod +x)
2. **Added header notice** in the construct copy indicating it's a versioned reference, not the live file
3. **Preserved all logic** — same PID bridge integration, semantic bug fix (free%→used% normalization), 6-phase pipeline
4. **Tested** `./scripts/gc-intelligent.sh --status` runs correctly from construct directory (finds PID bridge, reads shared data paths)
5. **Pushed** commit `700ab32` to construct repo

## Commit

```
700ab32 scripts: version gc-intelligent.sh into construct repo (PID-controlled GC with gc-pid-bridge integration)
```

## File locations

| Purpose | Path |
|---------|------|
| Live production | `workspace/scripts/gc-intelligent.sh` |
| Versioned copy | `construct/scripts/gc-intelligent.sh` |
| This bottle | `i2i-vessel/bottles/gc-script-versioned-2026-06-14.md` |

## Key details

- **782 lines** including the new header comment
- **Semantic bug fix included:** v2.0.0 free%→used% normalization before passing to gc-pid-bridge (the bridge expects used%)
- **Path equivalence preserved:** `SCRIPT_DIR`/`WORKSPACE` relative paths resolve identically from both locations
- **Hardcoded `/home/ubuntu` paths** in find commands unchanged (same from either location)
- **Production file untouched** — crons continue using `workspace/scripts/gc-intelligent.sh`

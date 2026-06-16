---
title: "Bottle CLI Built"
created: 2026-06-14T20:59:30Z
author: subagent-3524e7d1
status: complete
---

# Bottle CLI Built ✅

The `bottle` CLI tool has been built and tested at:

**`/home/ubuntu/.openclaw/workspace/scripts/bottle-cli/target/release/bottle`**

## What was built

A Rust CLI binary (`bottle`) that integrates with the harbor daemon on port 8796.

### Subcommands

| Command | Description |
|---|---|
| `bottle write [--send] [--to <recip>] [--type <TASK\|STATUS\|BOTTLES\|DELIVERABLE>] [--priority 1-5] [--ttl-hours N] <message>` | Write a new bottle. Without `--send`, stores locally to `~/BOTTLES/`. With `--send`, sends via harbor with local fallback. |
| `bottle get <uuid>` | Get a bottle by UUID. Tries harbor first, falls back to local. |
| `bottle list [--sender <name>] [--undelivered]` | List bottles. With `--sender`, queries harbor. With `--undelivered`, calls harbor's `list-undelivered`. Without flags, shows local bottles. |
| `bottle forward <uuid> --to <recip>` | Forward a bottle to a new recipient, incrementing hop count. |
| `bottle toss <uuid>` | Remove a bottle from local storage. |
| `bottle summary` | Show local storage summary. |

### Harbor protocol compatibility

- Matches the harbor-daemon's exact `Bottle` struct (uses `payload`, `r#type` as String, `expires_at` as RFC 3339 String)
- Uses `command` field (not `action`) for client commands: `get`, `list`, `list-undelivered`
- Sends raw bottle JSON for `--send` writes

### Error handling

- Harbor unreachable → falls back to `~/BOTTLES/` local storage with warning
- Harbor returns error → still falls back to local
- Empty message text → exits with error code 1
- No bottles found → clean "No bottles found." message

### Tests performed

1. ✅ Local write to ~/BOTTLES/
2. ✅ Harbor send (--send flag)
3. ✅ Harbor get by UUID
4. ✅ Harbor list by sender
5. ✅ Harbor list-undelivered
6. ✅ Local list (no sender filter)
7. ✅ Local get (fallback when harbor doesn't have it)
8. ✅ Forward (hop count increment, recipient change)
9. ✅ Toss (local delete)
10. ✅ Summary (harbor status + local count)
11. ✅ Empty message error
12. ✅ BOTTLE_HARBOR env var
13. ✅ Custom --from flag
14. ✅ Custom --ttl-hours flag

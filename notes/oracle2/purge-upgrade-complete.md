# Purge + Upgrade — Complete

**Date:** 2026-06-14 18:49 UTC
**Status:** ✅ Done

## OpenClaw Upgrade

**Before:** 2026.5.28 (8c802aa)
**After:**  2026.6.6 (8c802aa) ✅

Config snapshot saved to `/home/ubuntu/.openclaw/openclaw.json.pre-6.6.6`
Config unchanged (no migration needed) — `openclaw doctor --fix` was a no-op.

## Fleet Service Purge

### ✅ Disabled & Stopped (8 services)
| Service | Status |
|---------|--------|
| fleet-status (210K+ crash-loop) | ✅ disabled |
| fleet-modulation (exited) | ✅ disabled |
| fleet-matrix-bridge (exited) | ✅ disabled |
| fleet-opensmile (130MB RSS, CWD deleted) | ✅ disabled |
| midi-notebook-bridge (broken TTS endpoint) | ✅ disabled |
| piper-voice (dead) | ✅ disabled |
| fleet-ambient-briefing (timers) | ✅ disabled |
| fleet-watchdog (timers) | ✅ disabled |

### ✅ Already Cleaned (unit files gone)
| Service | Note |
|---------|------|
| fleet-ambient-loop | unit file deleted (was broken symlink) |
| fleet-health-monitor | unit file deleted (was broken symlink) |
| fleet-murmur-worker | unit file deleted (was broken symlink) |

### ⚠️ Manual Touch-Ups Needed (elevated)
- `sudo kill 2444140 2444151` — orphaned rotation-feed-server processes (19MB RSS)
- `sudo systemctl disable --now fleet-ambient-briefing.timer` — timer still has unit file
- `sudo systemctl disable --now fleet-watchdog.timer` — timer still has unit file

## Host Impact
| Metric | Before | After |
|--------|--------|-------|
| RAM used | ~2.5-3.0 GiB | 2.1 GiB (-400MB) |
| Disk used | ~25G | 25G (no change, file sizes were tiny) |
| Service files | 24 | ~15 (+ 3 gone) |

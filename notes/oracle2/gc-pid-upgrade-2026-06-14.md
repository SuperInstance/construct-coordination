# Bottle: GC PID Upgrade — 2026-06-14

## Summary
Fixed semantic bug in `gc-intelligent.sh` PID controller that was passing free% to a bridge expecting used%.

## Bug Found
`pid_calc()` in `gc-intelligent.sh` calls `gc-pid-bridge <disk_pct>` where `disk_pct` was historically the **free%** (computed as `(total - used) * 100 / total`). The bridge expects **used%** (setpoint=20 means "20% free space target", and PID error is `setpoint - current_usage`).

## Effect
At 63% disk used (37% free):
- **Before fix:** bridge receives 37 (free%), treats as 37% used, error = 20 - 37 = -17, aggression = 1.66x
- **After fix:** bridge receives 63 (used%), error = 20 - 63 = -43, aggression = 3.46x
- The bug inverted aggression: the more free space we had, the more aggressively we GC'd. Wrong.

## Fix
Added normalization in `pid_calc()`:
```bash
local bridge_input="$disk_pct"
if (( $(echo "$disk_pct <= 100" | bc -l) )); then
  bridge_input=$(echo "scale=2; 100 - $disk_pct" | bc)
fi
aggression=$("$PID_BRIDGE" "$bridge_input" 2>/dev/null)
```

## Verification
```
[gc-pid-bridge] /usr/local/bin/gc-pid-bridge
[gc-intelligent.sh] PID aggression: 1.66875x (before) → 3.45625x (after, correct)
```

## Files Changed
- `scripts/gc-intelligent.sh` — pid_calc() bridge call (line ~132-138)

## Not Committed
This file lives in `workspace/scripts/` (unversioned). The construct repo copy was added briefly for testing but removed. The fix is in the live production file.

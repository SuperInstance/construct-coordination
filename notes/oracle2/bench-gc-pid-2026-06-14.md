# Bottle: GC PID Benchmarks — 2026-06-14

## Summary
Benchmarked `gc-pid-bridge` (Rust+NEON) vs bash `bc` on same PID workload.

## Results (100 iterations per disk_pct)
| disk_pct | bridge (ms) | bc (ms) | speedup |
|----------|-------------|---------|---------|
| 5        | 109         | 1,418   | 13.0×   |
| 15       | 314         | 1,259   | 4.0×    |
| 30       | 97          | 1,419   | 14.6×   |
| 45       | 121         | 1,366   | 11.3×   |
| 50       | 122         | 1,900   | 15.6×   |
| 60       | 115         | 1,380   | 12.0×   |
| 70       | 115         | 1,335   | 11.6×   |
| 80       | 109         | 1,391   | 12.8×   |
| 85       | 146         | 1,393   | 9.5×    |
| 95       | 108         | 1,373   | 12.7×   |

**Average speedup: 8.8×**

## Config
- Hardware: ARM64 Neoverse-N1 (Oracle ARM)
- Bridge: gc-pid-bridge v1.2.0 (-C target-cpu=neoverse-n1)
- Baseline: bash bc (proportional-only, bc -l)
- Script: bench-gc-pid.sh in construct/scripts/
- Data: construct/data/bench-gc-pid.json

## Verdict
The ARM-native upgrade is justified. 8.8× average on pure math, but more importantly
the bridge adds PID features (I+D terms, deadband, anti-windup) that bc fallback can't do.

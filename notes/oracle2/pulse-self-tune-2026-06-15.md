# Bottle: Pulse Self-Tune Loop — 2026-06-15

## Summary
The construct stack now has a metabolic feedback loop. Every 5 minutes:

```
System metrics → conservation-meter → rotation-feed → harbor
                                           ↕
                              headspace-rs (vector embed)
                                           ↕
                              pulse-webhook (alert bottles)
                                           ↕
                              pulse-self-tune (GC setpoint adjustment)
```

## The Self-Tuner
`pulse-self-tune.sh` reads the γ/η ratio from conservation-meter and adjusts
the gc-pid-bridge setpoint dynamically:

| Ratio | State | Setpoint |
|-------|-------|----------|
| < 2   | Cool  | 20-40 (relaxed) |
| 2-4   | Nominal | 20 (default) |
| 4-6   | Stressed | 10-19 (tight) |
| > 6   | Critical | 10 (minimum) |
| Burn  | Meltdown | 10 (override) |

## Hardware
- headspace-rs: 4 segments stored, 384-dim hash embeddings
- Harbor: 19 bottles from full pipeline
- Conservation: 22 reports, ratio 1.83 (cool)
- GC aggression: 3.46x at current 63% used, setpoint=20

## Why This Matters
The stack is no longer a passive monitor. It measures, stores, alerts,
and adjusts — all in a 30-second feedback loop. The construct is alive.

# Forgiveness Sweep Findings

## Setup
- 100 instances × 1000 agents × 3000 ticks
- tunnel_rate: 0.006, trap_rate: 0.01
- forgiveness swept 0.0 to 1.0 in 100 steps

## Key Finding: The Forgiveness Paradox

**High forgiveness kills the system. Low forgiveness keeps it alive.**

This is counterintuitive but makes physical sense:

- **Forgiveness = entering 0 state when you disagree with a neighbor**
- High forgiveness → everyone drops to 0 on disagreement → spindle overload → death
- Low forgiveness → agents HOLD THEIR GROUND → more active agents → more survival
- BUT: zero forgiveness means agents never re-evaluate → rigid system

### The Curve
```
forgiveness  survival  entropy
0.00         0.378     1.335    ← Best survival AND entropy
0.01         0.363     1.308
0.05         0.249     1.056
0.10         0.224     0.989
0.20         0.185     0.849
0.30         0.161     0.741
0.50         0.130     0.598
0.80         0.119     0.449
1.00         0.102     0.337    ← Almost dead
```

### Why

In our model, forgiveness means "when I disagree with a neighbor, I enter the spindle (0) to think about it." But the spindle is a TRAP. Once you're in 0, you need tunneling to get out. The tunnel rate (0.006) is much lower than the forgiveness rate, so agents get trapped faster than they can escape.

**The lesson: Don't be too forgiving. Stand your ground. Enter the spindle sparingly.**

### Revised Understanding

The original trust-genome experiment found 0.5-0.7 forgiveness optimal because in that experiment, forgiveness also included TRUST REBUILDING (not just dropping to 0). In this simpler model, forgiveness IS dropping to 0, which is purely costly.

This tells us forgiveness has TWO components:
1. **Grace** — willingness to pause and reconsider (costly, enters trap)
2. **Trust rebuild** — willingness to cooperate again after conflict (beneficial, exits trap)

These need to be separate parameters. The grace component should be LOW (don't drop to 0 easily). The trust rebuild component should be MODERATE (do reconnect after conflict).

### Implication for ternary-engine

The engine should separate:
- `grace_rate` (how often you enter spindle on disagreement) → keep LOW
- `trust_rebuild_rate` (how fast trust recovers after conflict) → keep MODERATE
- `tunnel_rate` (how easily you exit spindle) → keep >= 0.003

Current `forgiveness` conflates grace and trust rebuild. They're opposite forces.

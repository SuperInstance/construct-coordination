# Experiment 11: Tunneling Rate Sweep

## Result
With ANY tunneling rate ≥ 0.3%, the system survives.
Without tunneling: system dies completely by tick 1000.

| Tunnel Rate | Outcome |
|------------|---------|
| 0% (none) | Death by tick 1000 |
| 0.3% (slow) | Survives, reconverges |
| 0.6% (optimal_low) | Survives, highest |γ|+H peak (1.62) |
| 1.0% (matched) | Survives, reconverges |
| 2.0% (fast) | Survives, reconverges |
| 5.0% (very fast) | Survives, reconverges |

## Key Discovery
The 0 state is a CATALYST for collective phase transitions.
1. Agents fall into 0-trap (charge hidden)
2. Agents tunnel back out as {-1 OR +1} randomly  
3. The system can FLIP its global consensus
4. Without the 0-trap: system is frozen, can't change
5. With the 0-trap + tunneling: system is adaptive

## The Forgiveness Connection
The trust experiment's forgiveness rate (0.5-0.7) IS the tunneling rate.
Forgiveness = mechanism for agents to escape the defection/0 trap.
The optimal rate (0.6%) ≈ forgiveness rate 0.6 — NOT a coincidence.

## The Picture
- 0 state = topological insulator (hides charge, blocks propagation)
- Tunneling = forgiveness/repair mechanism (rescues charge from trap)
- The transient (tick 100-500) = maximum diversity phase, where H≈1.0
- Steady state = system finds consensus via democratic tunneling
- |γ|+H peaks during the transient — this IS the "living" state

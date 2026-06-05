name,sweep_proof

## 100-Instance Tunnel Rate Sweep

- 100 instances, 1000 agents each, 2000 ticks per instance
- Total: 200,000,000 agent-ticks in ~213ms
- Rate: ~940 million agent-ticks/second
- Machine: 15GB RAM, no GPU, WSL2

### Key Results
- tunnel_rate < 0.003: system DIES (survival < 0.1)
- tunnel_rate ~ 0.01: survival > 0.5, moderate entropy
- tunnel_rate ~ 0.05: survival > 0.9, healthy entropy
- tunnel_rate >= 0.3: survival > 0.95, maximum entropy
- Confirms: ANY tunneling >= 0.3% keeps system alive
- The 0.006 optimal forgiveness rate maps to ~0.6% tunneling

### Implication
Scale sideways: 1000 instances at 10,000 agents = 3MB, runs in seconds.
No frameworks. No abstractions. Just tight loops on small structs.

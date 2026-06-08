# A2A Modules — Fleet Cross-Reference

## Module → Fleet Repo Mapping

| A2A Module | Connects To | Type |
|-----------|-------------|------|
| WASM kernel | fleet-ternary-music, fleet-arm-compat | Core math in binary form |
| Spectral→MIDI | spectral-spreadsheet, fleet-music-theorist | Graph analysis → chord voicing |
| Bridge Protocol | spreadsheet-engine, i2i-bottle-agent, fleet-bridge | Protocol translation |
| Pipeline A2A | superinstance-spreadsheet, all MIDI repos | CSV→MIDI pipeline |

## Fleet Repo → Module Mapping

| Fleet Repo | Uses A2A Module | Purpose |
|-----------|----------------|---------|
| fleet-ternary-music | WASM kernel | Reference implementation |
| fleet-arm-compat | WASM kernel | ARM64 verification target |
| spectral-spreadsheet | Spectral→MIDI | Bridge graph analysis to MIDI |
| fleet-music-theorist | Spectral→MIDI | Deeper harmonic analysis |
| spreadsheet-engine | Bridge Protocol | Convert cells to agents |
| i2i-bottle-agent | Bridge Protocol | Convert bottles to cells |
| fleet-bridge | Bridge Protocol | Cross-protocol routing |
| superinstance-spreadsheet | Pipeline A2A | Strategy→MIDI pipeline |
| fleet-orchestra | Pipeline A2A | Intent routing to MIDI generation |
| construct-coordination | All | Fleet communication hub |

## Dual Architecture Cross-Reference

```
I2I Fleet (message passing)          Spreadsheet Fleet (functional composition)
─────────────────────────────         ───────────────────────────────────────
fleet-bridge                          spreadsheet-engine
fleet-orchestra                       spreadsheet-cells
i2i-bottle-agent                      spreadsheet-formulas
tminus-dispatcher                     ternary-spreadsheet
tminus-client                         ternary-spreadsheet-python
symphony-runtime                      ternary-spreadsheet-c
composite-headspace                   spectral-spreadsheet
    │                                     │
    └─────────── Bridge Protocol ──────────┘
```

## External References

- fleet-architecture: Layer 5 (A2A module layer)
- fleet-science: Papers on Z₃ group and Neo-Riemannian connections
- fleet-tutorials: Walk-throughs using Pipeline A2A

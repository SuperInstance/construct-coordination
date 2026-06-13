# 📊 Spreadsheet Ecosystem — Complete Map

## 12 Repos Found

| Repo | Language | What it does | Fleet Counterpart |
|------|----------|-------------|-------------------|
| superinstance-spreadsheet | HTML+Py | Browser UI + GPU backend (Torch) | fleet-midi-text2midi |
| spreadsheet-engine | Rust | Core engine (crates.io!) — every cell = agent or MIDI | fleet-orchestra |
| spreadsheet-cells | Python | Multi-agent fleet cell simulation | i2i-bottle-agent |
| spreadsheet-formulas | Rust | Formula parser for cells | fleet-midi-pattern |
| ternary-spreadsheet | Python | Core ternary cell logic | fleet-ternary-music |
| ternary-spreadsheet-python | Python | Python ternary spreadsheet | fleet-ternary-music |
| ternary-spreadsheet-c | C | C implementation | fleet-ternary-music |
| Spreadsheet-ai | - | AI formula generation | fleet-midi-text2midi |
| spectral-spreadsheet | HTML | Browser spectral analysis | fleet-music-theorist |
| spreadsheet-moment-proto | - | Moment-based proto | fleet-ternary-music |
| spreadsheet-projection | - | Projection ops | fleet-osc-server |

## Dual Architecture Insight

Two complementary approaches to the same problem:

**Our Fleet (I2I bottles):** Agents communicate via structured messages
**Spreadsheet Fleet (cell formulas):** Cells communicate via functional composition

Both use the same ternary {-1,0,+1} values. Both solve multi-agent coordination.
They are dual architectures for the same coordination problem.

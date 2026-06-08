# 🎵 SuperInstance MIDI Fleet — Content Hub

> *97 demo files, 24 tutorials, 10 production-grade repos*

## Repos

| # | Repo | Ensign | Demos | Tutorials |
|---|------|--------|-------|-----------|
| 1 | [text2midi](https://github.com/SuperInstance/fleet-midi-text2midi) | Rhapsodia 🎹 | 12 MIDI files | 4 tutorials |
| 2 | [tidalcycles](https://github.com/SuperInstance/fleet-midi-tidalcycles) | Rhythmica 🥁 | 12 patterns | 4 tutorials |
| 3 | [musiclang](https://github.com/SuperInstance/fleet-midi-musiclang) | Harmonia 🎵 | 12 progressions | 4 tutorials |
| 4 | [generator](https://github.com/SuperInstance/fleet-midi-generator) | Composita 🧠 | 12 completions | 4 tutorials |
| 5 | [tokenizer](https://github.com/SuperInstance/fleet-midi-tokenizer) | Glyph 🔤 | 12 roundtrips | 4 tutorials |
| 6 | [markov](https://github.com/SuperInstance/fleet-midi-markov) | Weaver 🕸️ | 12 generations | 4 tutorials |
| 7 | [symusic](https://github.com/SuperInstance/fleet-midi-symusic) | Anvil ⚒️ | — | — |
| 8 | [sonicpi](https://github.com/SuperInstance/fleet-midi-sonicpi) | Pulse 💫 | — | — |
| 9 | [foxdot](https://github.com/SuperInstance/fleet-midi-foxdot) | Sprite 🦊 | — | — |
| 10 | [juce](https://github.com/SuperInstance/fleet-midi-juce) | Anvil ⚒️ | — | — |

## Directories

- `text2midi/` — 12 MIDI demos + 4 tutorials + ONBOARDING
- `tidalcycles/` — 12 pattern demos + 4 tutorials
- `musiclang/` — 12 chord progressions + 4 tutorials
- `generator/` — 12 MIDI completions + 4 tutorials
- `tokenizer/` — 24 tokenizations + 4 tutorials
- `markov/` — 12 generations + 4 tutorials
- `reports/` — Structured data reports

## Fleet Pipeline

```
Text Prompt → Rhapsodia → MIDI File → Glyph → REMI Tokens
                                                         ↓
Ternary Vector → Rhythmica → Pattern → Harmonia → Arrangement
                                                         ↓
State Sequence → Composita → MIDI Completion ← Weaver → Markov
                                                         ↓
Pulse → Sonic Pi live_loop      Sprite → FoxDot OSC
```

## Per-Repo Production Documentation

Each repo has:
- **README.md** — Award-winning with ASCII diagrams + badge bar
- **ONBOARDING.md** — Dual-audience: humans + agent ensigns
- **AGENT.md** — Ensign identity with summon command
- **memory/JOURNAL.md** — Duty log
- **Dockerfile** — Ready to build and deploy
- **.github/workflows/ci.yml** — CI pipeline
- **.github/workflows/publish.yml** — npm publish on release
- **tests/zeroshot.sh** — Zeroshot test suite

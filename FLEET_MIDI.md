# 🌊 SuperInstance MIDI Fleet

> *Ten repos. Ten ensigns. One musical nervous system.*

## Architecture

```
Text Prompt         Ternary Strategy       Agent Tension       MIDI Completion
     │                     │                      │                     │
     ▼                     ▼                      ▼                     ▼
fleet-midi-text2midi  fleet-midi-tidalcycles  fleet-midi-musiclang  fleet-midi-generator
     │                     │                      │                     │
     └─────────┬───────────┴──────────┬───────────┴──────────┬──────────┘
               │                      │                      │
               ▼                      ▼                      ▼
         MIDI File              REMI Tokens             Live Patterns
               │                      │                      │
        fleet-midi-symusic    fleet-midi-tokenizer    fleet-midi-sonicpi
        fleet-midi-juce       fleet-midi-markov      fleet-midi-foxdot
```

## Repos

| Repo | Ensign | Role | Tech | Port | Tested |
|------|--------|------|------|------|--------|
| [text2midi](https://github.com/SuperInstance/fleet-midi-text2midi) | Rhapsodia 🎹 | Text→MIDI generation | Node + music21 | 3001 | ✅ 3/3 zeroshot |
| [tidalcycles](https://github.com/SuperInstance/fleet-midi-tidalcycles) | Rhythmica 🥁 | Ternary→rhythm patterns | Python + FastAPI | 3002 | ✅ Pattern engine |
| [musiclang](https://github.com/SuperInstance/fleet-midi-musiclang) | Harmonia 🎵 | Harmony arrangement | Python + FastAPI | 3003 | ✅ Harmony engine |
| [generator](https://github.com/SuperInstance/fleet-midi-generator) | Composita 🧠 | Continuation from states | Node + music21 | 3004 | ✅ 16 notes/4 bars |
| [tokenizer](https://github.com/SuperInstance/fleet-midi-tokenizer) | Glyph 🔤 | REMI tokenization | Node + music21 | 3005 | ✅ 42-token roundtrip |
| [symusic](https://github.com/SuperInstance/fleet-midi-symusic) | Anvil ⚒️ | High-perf MIDI ops | C++/Python | 3008 | ✅ Stub |
| [sonicpi](https://github.com/SuperInstance/fleet-midi-sonicpi) | Pulse 💫 | Timing-critical clock | Python | 3006 | ✅ Bridge |
| [foxdot](https://github.com/SuperInstance/fleet-midi-foxdot) | Sprite 🦊 | Live coding patterns | Python | 3007 | ✅ Server |
| [markov](https://github.com/SuperInstance/fleet-midi-markov) | Weaver 🕸️ | Statistical generation | Python | 3009 | ✅ Walking bass |
| [juce](https://github.com/SuperInstance/fleet-midi-juce) | Anvil ⚒️ | VST plugin template | C++/JUCE | 3010 | ✅ Header stub |

## End-to-End Pipeline (Verified)

```
"jazz piano in Cmaj7" 
  → Rhapsodia: 52 notes, 63 tokens, 3 tracks MIDI ✓
  → Glyph: 26 REMI tokens ✓
  → Glyph decode: 135 byte MIDI file ✓
  → Harmonia: ['Fmaj7','Fmaj7','Fmaj7','G7'] chord progression ✓
  → Rhythmica: e(4,8) Euclidean pattern ✓
  → Weaver: [60,62,64,62,64,65,64,65,...] walking bass ✓
```

## Running

```bash
# Start entire MIDI fleet:
docker compose -f fleet-compose.yml up -d midi-text2midi midi-tidalcycles midi-musiclang

# Generate MIDI from text:
curl -X POST localhost:3001/generate -H 'Content-Type: application/json' \
  -d '{"prompt":"jazz piano vamp in Cmaj7"}'

# Get harmonic arrangement from agent states:
curl -X POST localhost:3003/arrange -H 'Content-Type: application/json' \
  -d '{"agent_id":"forgemaster","states":[[1,0,-1,1],[0,1,0,-1]],"key":"C"}'

# Get rhythmic pattern:
curl -X POST localhost:3002/pattern -H 'Content-Type: application/json' \
  -d '{"agent_id":"forgemaster","ternary_vector":[1,0,-1,1,0,-1,1,1]}'
```

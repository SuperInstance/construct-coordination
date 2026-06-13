# 🔗 Quick Cross-Pollination Chains

Copy-paste these to connect SuperInstance repos:

## Chain A: ternary math → music
```bash
# Analyze a pattern with ternary tools
python3 -c "
v = [1,0,-1,1,0,-1,1,1]
print('ternary-rhythm would analyze:', v)
print('tidalcycles would render:', 's \"bd\", s \"hh\", s \"sn\"')
print('text2midi would generate MIDI')
"
```

## Chain B: I2I bottle → MIDI transport
```bash
# Send a MIDI bottle through the fleet
curl -X POST localhost:4000/bottle \
  -d '{"type":"MIDI_DELIVERABLE","payload":{"notes":[60,64,67]}}'
```

## Chain C: Symphony → full arrangement
```bash
# Dispatch timed musical events
curl -X POST localhost:4001/dispatch \
  -d '{"agent":"composita","states":[[1,0,-1],[0,1,-1]],"tempo":120}'
```

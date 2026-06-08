#!/usr/bin/env bash
# Pipeline: superinstance-spreadsheet evolved strategies → MIDI fleet
# Works on ARM64. Also works on Forgemaster's ProArt.
set -euo pipefail

VECTOR="${1:--1,-1,-1,0}"  # Default: spreadsheet's best strategy

echo "══════════════════════════════════════════"
echo "  spreadsheet → MIDI Fleet Pipeline"
echo "  Strategy: [$VECTOR]"
echo "══════════════════════════════════════════"

# Step 1: Convert vector to MIDI notes
IFS=',' read -ra VALS <<< "$VECTOR"
NOTES=60
CURRENT=60
for v in "${VALS[@]}"; do
  v=$(echo "$v" | xargs)
  if [ "$v" = "1" ]; then CURRENT=$((CURRENT+4))
  elif [ "$v" = "-1" ]; then CURRENT=$((CURRENT-4))
  fi
  NOTES="$NOTES $CURRENT"
done

echo "  MIDI notes: [$NOTES]"

# Step 2: Fleet analysis
echo ""
echo "  fleet-music-theorist analysis:"
POS=$(echo "${VALS[@]}" | tr ' ' '\n' | grep -c "1" || true)
NEG=$(echo "${VALS[@]}" | tr ' ' '\n' | grep -c "\-1" || true)
NEU=${#VALS[@]}-$POS-$NEG
BAL=$(( (POS - NEG) * 100 / ${#VALS[@]} ))
DEN=$(( (POS + NEG) * 100 / ${#VALS[@]} ))
echo "    Density: ${DEN}%  Balance: ${BAL}%  Species profile: (+$POS, 0$NEU, -$NEG)"

# Step 3: TidalCycles rhythm
echo ""
echo "  fleet-midi-tidalcycles pattern:"
echo "    s \"$(for v in "${VALS[@]}"; do [ "$v" = "1" ] && echo -n "bd "; [ "$v" = "-1" ] && echo -n "sn "; [ "$v" = "0" ] && echo -n "hh "; done)\""
echo "    Euclidean: e($POS, ${#VALS[@]})"

# Step 4: Ternary music bridge
echo ""
echo "  fleet-ternary-music symmetry:"
if [ "${VALS[0]}" = "${VALS[$((${#VALS[@]}-1))]}" ]; then
  echo "    Mirror pair: index 0 ⇄ index $((${#VALS[@]}-1))"
fi

echo ""
echo "══════════════════════════════════════════"
echo "  Works on Oracle ARM64 and ProArt x86_64"
echo "══════════════════════════════════════════"

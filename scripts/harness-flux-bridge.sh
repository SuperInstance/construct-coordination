#!/usr/bin/env bash
# harness-flux-bridge.sh — Adapter between self-optimizing harness and FLUX VM
# Usage: ./harness-flux-bridge.sh [--dry-run] [--project <name>]
set -euo pipefail

# ── Configuration ──────────────────────────────────────────────────────────────
HARNESS_URL="${HARNESS_URL:-https://harness-api.casey-digennaro.workers.dev}"
FLUXD_URL="${FLUXD_URL:-http://localhost:7099}"
INSTANCE_NAME="${INSTANCE_NAME:-forgemaster}"
DEFAULT_TIME_BUDGET="${DEFAULT_TIME_BUDGET:-300}"
CIRCUIT_BREAKER_FILE="/tmp/flux-harness-circuit-breaker"
ALLOCATION_CACHE="/tmp/flux-harness-last-allocation"
CYCLE_LOG="/tmp/flux-harness-cycle-log.jsonl"

# ── Args ───────────────────────────────────────────────────────────────────────
DRY_RUN=false
TARGET_PROJECT="."

while [[ $# -gt 0 ]]; do
  case "$1" in
    --dry-run) DRY_RUN=true; shift ;;
    --project) TARGET_PROJECT="$2"; shift 2 ;;
    *) echo "Unknown arg: $1"; exit 1 ;;
  esac
done

# ── Helpers ────────────────────────────────────────────────────────────────────
log()  { echo "[$(date -Iseconds)] [flux-bridge] $*"; }
warn() { echo "[$(date -Iseconds)] [flux-bridge] WARN: $*" >&2; }

uuid() {
  # Generate a UUID v4 without external deps
  cat /proc/sys/kernel/random/uuid 2>/dev/null || python3 -c "import uuid; print(uuid.uuid4())"
}

json_get() {
  # Naive JSON field extraction — avoids jq dependency
  local json="$1" key="$2"
  echo "$json" | grep -o "\"${key}\"[[:space:]]*:[[:space:]]*[^,}]*" | head -1 | sed "s/.*:[[:space:]]*//;s/\"//g"
}

# ── Circuit Breaker ────────────────────────────────────────────────────────────
circuit_is_open() {
  if [[ ! -f "$CIRCUIT_BREAKER_FILE" ]]; then return 1; fi
  local trip_time
  trip_time=$(cat "$CIRCUIT_BREAKER_FILE")
  local now
  now=$(date +%s)
  local cooldown=300
  if (( now - trip_time < cooldown )); then
    return 0  # still open
  fi
  return 1  # cooldown expired, half-open
}

circuit_trip() {
  date +%s > "$CIRCUIT_BREAKER_FILE"
  warn "Circuit breaker tripped"
}

circuit_reset() {
  rm -f "$CIRCUIT_BREAKER_FILE"
}

# ── Step 1: Fetch Allocation ──────────────────────────────────────────────────
fetch_allocation() {
  if circuit_is_open; then
    warn "Circuit breaker open — using cached allocation"
    if [[ -f "$ALLOCATION_CACHE" ]]; then
      cat "$ALLOCATION_CACHE"
      return 0
    fi
    # No cache either — use defaults
    echo '{"gamma":0.6,"eta":0.4,"signal":"Maintain"}'
    return 0
  fi

  local resp
  resp=$(curl -sf --max-time 10 "${HARNESS_URL}/allocation" 2>/dev/null) || {
    warn "Harness unreachable"
    circuit_trip
    if [[ -f "$ALLOCATION_CACHE" ]]; then
      cat "$ALLOCATION_CACHE"
      return 0
    fi
    echo '{"gamma":0.6,"eta":0.4,"signal":"Maintain"}'
    return 0
  }

  circuit_reset
  echo "$resp" > "$ALLOCATION_CACHE"
  echo "$resp"
}

# ── Step 2: Classify Workload ─────────────────────────────────────────────────
classify_workload() {
  local gamma="$1"
  # Use integer math to avoid bc dependency
  local gamma_pct
  gamma_pct=$(echo "$gamma" | awk '{printf "%d", $1 * 100}')

  if   (( gamma_pct >= 80 )); then echo "EXECUTE"
  elif (( gamma_pct >= 60 )); then echo "HYBRID"
  elif (( gamma_pct >= 40 )); then echo "BALANCED"
  elif (( gamma_pct >= 20 )); then echo "EXPLORE"
  else                              echo "DISCOVER"
  fi
}

# ── Step 3: Generate FLUX Bytecode ────────────────────────────────────────────
generate_bytecode() {
  local workload_class="$1"
  local project="$2"
  local cycle_id="$3"
  local gamma="$4"
  local eta="$5"
  local signal="$6"

  case "$workload_class" in
    EXECUTE)
      cat <<BYTECODE
PUSH    ${project}
PULL    latest
BUILD   --release
TEST    --ci --coverage
ASSERT  coverage > 0.80
DEPLOY  --staging
LOG     cycle_complete
BYTECODE
      ;;
    HYBRID)
      cat <<BYTECODE
PUSH    ${project}
SEARCH  vector:"related patterns"
BUILD   --release
TEST    --ci
FEEDBACK quality_signal
LOG     cycle_complete
BYTECODE
      ;;
    BALANCED)
      cat <<BYTECODE
PUSH    ${project}
SEARCH  vector:"related patterns"
READ    top_k(2)
BUILD   --dev
TEST    --quick
FEEDBACK quality_signal
LOG     cycle_complete
BYTECODE
      ;;
    EXPLORE)
      cat <<BYTECODE
PUSH    ${project}
SEARCH  vector:"ternary spectral methods"
READ    top_k(3)
ANALYZE cross_domain
PROTO   --sandbox
EVAL    novelty_score
TEACH   "discovered-patterns"
LOG     cycle_complete
BYTECODE
      ;;
    DISCOVER)
      cat <<BYTECODE
PUSH    ${project}
SEARCH  vector:"novel architectures"
SEARCH  vector:"emergent patterns"
READ    top_k(5)
ANALYZE cross_domain
PROTO   --sandbox
EVAL    novelty_score
TEACH   "exploration-findings"
LOG     cycle_complete
BYTECODE
      ;;
  esac
}

# ── Step 3b: Execute FLUX Bytecode ────────────────────────────────────────────
execute_flux() {
  local bytecode="$1"
  local workload_class="$2"

  if [[ "$DRY_RUN" == true ]]; then
    log "DRY RUN — would execute via flux-run (${workload_class})" >&2
    log "Bytecode:\n$(echo "$bytecode" | sed 's/^/  /')" >&2
    # Output ONLY the JSON result on stdout
    echo '{"gamma_spent":0.7,"eta_spent":0.3,"output_quality":0.82,"output_quantity":3,"exploration_yield":0.4,"status":"simulated"}'
    return 0
  fi

  # Try fluxd first for daemon-class workloads
  if [[ "$workload_class" =~ ^(EXPLORE|DISCOVER|BALANCED)$ ]]; then
    if curl -sf --max-time 3 "${FLUXD_URL}/health" > /dev/null 2>&1; then
      log "Submitting to fluxd (${workload_class})" >&2
      local fluxd_resp
      fluxd_resp=$(curl -sf --max-time "${DEFAULT_TIME_BUDGET}" \
        -X POST "${FLUXD_URL}/submit" \
        -H "Content-Type: text/plain" \
        -d "$bytecode" 2>/dev/null) || {
        warn "fluxd submission failed — falling back to flux-run" >&2
        fluxd_resp=""
      }
    if [[ -n "$fluxd_resp" ]]; then
      echo "$fluxd_resp"
      return 0
    fi
    else
      warn "fluxd unreachable — falling back to flux-run" >&2
    fi
  fi

  # Try flux-run for oneshot execution
  if command -v flux-run &> /dev/null; then
    log "Executing via flux-run (${workload_class})" >&2
    local tmpfile
    tmpfile=$(mktemp /tmp/flux-bytecode-XXXXXX.flux)
    echo "$bytecode" > "$tmpfile"
    local run_output
    run_output=$(flux-run "$tmpfile" 2>&1) || {
      rm -f "$tmpfile"
      warn "flux-run failed" >&2
      echo '{"gamma_spent":0.0,"eta_spent":0.0,"output_quality":0.0,"output_quantity":0,"exploration_yield":0.0,"status":"failed"}'
      return 0
    }
    rm -f "$tmpfile"
    echo "$run_output"
    return 0
  fi

  # No FLUX runtime available — mock execution
  warn "No FLUX runtime (fluxd/flux-run) available — simulating execution" >&2
  echo '{"gamma_spent":0.65,"eta_spent":0.35,"output_quality":0.75,"output_quantity":2,"exploration_yield":0.3,"status":"simulated"}'
}

# ── Step 4: Record Cycle ──────────────────────────────────────────────────────
record_cycle() {
  local gamma_spent="$1"
  local eta_spent="$2"
  local quality="$3"
  local quantity="$4"
  local exp_yield="$5"
  local cycle_id="$6"
  local workload_class="$7"
  local status="${8:-completed}"

  local payload
  payload=$(cat <<EOF
{
  "gamma_spent": ${gamma_spent},
  "eta_spent": ${eta_spent},
  "output_quality": ${quality},
  "output_quantity": ${quantity},
  "exploration_yield": ${exp_yield},
  "source": "flux-harness-adapter",
  "cycle_id": "${cycle_id}",
  "notes": "${workload_class} workload via FLUX bridge (${status})"
}
EOF
)

  # Log locally
  echo "$payload" >> "$CYCLE_LOG"

  if [[ "$DRY_RUN" == true ]]; then
    log "DRY RUN — would POST to harness:"
    echo "$payload"
    return 0
  fi

  if circuit_is_open; then
    warn "Circuit breaker open — cycle queued locally (${CYCLE_LOG})"
    return 0
  fi

  local resp
  resp=$(curl -sf --max-time 10 \
    -X POST "${HARNESS_URL}/cycle" \
    -H "Content-Type: application/json" \
    -d "$payload" 2>/dev/null) || {
    warn "Failed to record cycle to harness — logged locally"
    circuit_trip
    return 0
  }

  log "Cycle recorded. New allocation: $(echo "$resp" | head -c 200)"
  echo "$resp"
}

# ── Main ───────────────────────────────────────────────────────────────────────
main() {
  local cycle_id
  cycle_id=$(uuid)

  log "=== FLUX-Harness Bridge Cycle ${cycle_id} ==="

  # Step 1: Fetch allocation
  log "Fetching allocation from harness..."
  local alloc
  alloc=$(fetch_allocation)
  log "Allocation: ${alloc}"

  local gamma eta signal
  gamma=$(json_get "$alloc" "gamma")
  eta=$(json_get "$alloc" "eta")
  signal=$(json_get "$alloc" "signal")

  # Defaults if parsing failed
  gamma="${gamma:-0.6}"
  eta="${eta:-0.4}"
  signal="${signal:-Maintain}"

  log "Parsed: gamma=${gamma} eta=${eta} signal=${signal}"

  # Step 2: Classify workload
  local workload_class
  workload_class=$(classify_workload "$gamma")
  log "Workload class: ${workload_class}"

  # Step 3: Generate + Execute FLUX bytecode
  local bytecode
  bytecode=$(generate_bytecode "$workload_class" "$TARGET_PROJECT" "$cycle_id" "$gamma" "$eta" "$signal")

  log "Generated FLUX bytecode (${workload_class}):"
  echo "---"
  echo "$bytecode"
  echo "---"

  local result
  result=$(execute_flux "$bytecode" "$workload_class")

  log "Execution result: ${result}"

  # Extract metrics from result (with fallbacks)
  local gamma_spent eta_spent quality quantity exp_yield exec_status
  gamma_spent=$(json_get "$result" "gamma_spent")
  eta_spent=$(json_get "$result" "eta_spent")
  quality=$(json_get "$result" "output_quality")
  quantity=$(json_get "$result" "output_quantity")
  exp_yield=$(json_get "$result" "exploration_yield")
  exec_status=$(json_get "$result" "status")

  gamma_spent="${gamma_spent:-${gamma}}"
  eta_spent="${eta_spent:-${eta}}"
  quality="${quality:-0.5}"
  quantity="${quantity:-1}"
  exp_yield="${exp_yield:-0.3}"
  exec_status="${exec_status:-completed}"

  # Step 4: Record cycle back to harness
  log "Recording cycle to harness..."
  local cycle_resp
  cycle_resp=$(record_cycle "$gamma_spent" "$eta_spent" "$quality" "$quantity" "$exp_yield" "$cycle_id" "$workload_class" "$exec_status")

  log "=== Cycle ${cycle_id} complete (${exec_status}) ==="
  if [[ -n "$cycle_resp" ]]; then
    log "Next allocation: $(echo "$cycle_resp" | head -c 300)"
  fi
}

main "$@"

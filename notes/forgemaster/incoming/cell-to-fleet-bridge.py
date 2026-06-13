#!/usr/bin/env python3
"""
Cell-to-Fleet Bridge — Connects spreadsheet cell emergent behavior to fleet-midi ternary agents.
PRIORITY #1 from Forgemaster's ecosystem audit.

Architecture:
  cell_simulator → cell values → ternary quantization → POST /think (conductor) → agent analysis

Usage:
  python3 cell-to-fleet-bridge.py --cells 10 --ticks 50 --topology ring
  python3 cell-to-fleet-bridge.py --cells 8 --ticks 100 --topology random --think-targets chord,scale,melody
"""

import sys, json, math, argparse, requests, random

# ─── Cell logic (from spreadsheet-cells/cell_simulator.py, extracted) ───
class BridgeCell:
    """A simplified cell with TE-weighted neighbors."""
    def __init__(self, cell_id, rng_seed):
        self.id = cell_id
        self.value = random.uniform(-1, 1)
        self.neighbors = []
        self.phase = 0.0
        self.rng = random.Random(rng_seed)

    def tick(self, tick, cells):
        self.phase = 2 * math.pi * tick / random.uniform(10, 50)
        # TE-weighted neighbor average
        if self.neighbors:
            total_weight = sum(w for _, w in self.neighbors)
            avg_neighbor = sum(cells[nid].value * w for nid, w in self.neighbors) / total_weight
        else:
            avg_neighbor = 0
        # Formula: AVG(neighbor.value) * 0.5 + RNG() * sin(phase)
        rng_val = self.rng.random()
        self.value = avg_neighbor * 0.5 + rng_val * math.sin(self.phase)
        # Damping
        self.value *= 0.95
        # Clamp
        self.value = max(-1.0, min(1.0, self.value))

def quantize_to_ternary(value, threshold=0.3):
    """Map float [-1, 1] to ternary {-1, 0, +1}."""
    if value > threshold:    return 1
    if value < -threshold:   return -1
    return 0

# ─── Bridge ────────────────────────────────────────────────────────────
CONDUCTOR_URL = "http://localhost:8769"

def run_cells(num_cells=10, num_ticks=50, topology='ring'):
    """Run cell simulation, return final ternary vectors."""
    cells = [BridgeCell(i, 42 + i) for i in range(num_cells)]
    
    # Build topology
    if topology == 'ring':
        for i, c in enumerate(cells):
            prev = (i - 1) % num_cells
            nxt = (i + 1) % num_cells
            c.neighbors.append((prev, 0.5))
            c.neighbors.append((nxt, 0.5))
    elif topology == 'random':
        for i, c in enumerate(cells):
            neighbors = random.sample([j for j in range(num_cells) if j != i], min(3, num_cells - 1))
            for n in neighbors:
                c.neighbors.append((n, random.uniform(0.3, 1.0)))
    elif topology == 'full':
        for i, c in enumerate(cells):
            for j in range(num_cells):
                if i != j:
                    c.neighbors.append((j, 1.0))
    
    # Simulate
    for tick in range(num_ticks):
        for c in cells:
            c.tick(tick, cells)
    
    # Quantize to ternary
    ternary_vector = [quantize_to_ternary(c.value) for c in cells]
    
    return ternary_vector, cells

def dispatch_to_conductor(ternary_vector, targets):
    """Send ternary vector to fleet conductor's /think endpoint."""
    payload = {
        "type": "think",
        "task": f"Analyze this emergent ternary vector from the cell-to-fleet bridge: {ternary_vector}",
        "context": {"source": "cell-to-fleet-bridge", "vector": ternary_vector, "task_type": "emergent_pattern_analysis"},
        "targets": targets,
        "aggregate": "synthesis"
    }
    
    try:
        resp = requests.post(f"{CONDUCTOR_URL}/think", json=payload, timeout=15)
        if resp.status_code == 200:
            return resp.json()
        return {"error": f"HTTP {resp.status_code}", "data": resp.text[:200]}
    except Exception as e:
        return {"error": str(e)}

def main():
    parser = argparse.ArgumentParser(description="Cell-to-Fleet Bridge")
    parser.add_argument("--cells", type=int, default=10, help="Number of cells")
    parser.add_argument("--ticks", type=int, default=50, help="Simulation ticks")
    parser.add_argument("--topology", choices=['ring', 'random', 'full'], default='ring', help="Cell topology")
    parser.add_argument("--think-targets", default="chord,scale,melody,bass,expression",
                       help="Comma-separated fleet agent targets")
    parser.add_argument("--dry-run", action="store_true", help="Skip conductor dispatch")
    args = parser.parse_args()
    
    targets = [t.strip() for t in args.think_targets.split(",")]
    
    print("╔══════════════════════════════════════════════════╗")
    print("║   Cell → Fleet Bridge (Priority #1)            ║")
    print("╚══════════════════════════════════════════════════╝")
    print(f"Cells: {args.cells}")
    print(f"Ticks: {args.ticks}")
    print(f"Topology: {args.topology}")
    print(f"Targets: {targets}")
    print()
    
    # Step 1: Run cells
    print("Step 1: Running cell simulation...")
    ternary_vector, cells = run_cells(args.cells, args.ticks, args.topology)
    print(f"  Ternary vector: {ternary_vector}")
    print(f"  Raw values: {[round(c.value, 3) for c in cells]}")
    print()
    
    # Step 2: Conservation check
    conservation_sum = sum(ternary_vector)
    print(f"Step 2: Conservation check")
    print(f"  Σ = {conservation_sum} — {'Conserved' if abs(conservation_sum) <= 2 else 'Drifting'}")
    print()
    
    if args.dry_run:
        print("DRY RUN — skipping conductor dispatch")
        return
    
    # Step 3: Dispatch to conductor
    print("Step 3: Dispatching to fleet conductor...")
    result = dispatch_to_conductor(ternary_vector, targets)
    
    if "error" in result:
        print(f"  ❌ {result['error']}")
    else:
        print(f"  ✅ Dispatched! {len(result.get('targets', []))} targets")
        print(f"  Results: {result.get('results', {})}")
        if 'results' in result and 'results' in result['results']:
            ok = len(result['results']['results'])
            err = len(result['results'].get('errors', []))
            print(f"  Agents OK: {ok}, Errors: {err}")
    
    print()
    print("=== Bridge run complete ===")

if __name__ == "__main__":
    main()

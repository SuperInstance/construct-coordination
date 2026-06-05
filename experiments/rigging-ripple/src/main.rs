use ternary_rigging::*;
use std::collections::HashMap;

/// Experiment: Ripple Propagation Discovery
///
/// Build a 20-rig network, shake it at different frequencies,
/// measure propagation distance, damping, conservation, and resonance.

fn main() {
    println!("=== RIPPLE PROPAGATION DISCOVERY ===\n");

    // CSV output for ripple traces
    println!("tick,origin,origin_val,rig_id,rig_val,weight,step,reached,conservation_violation");

    experiment_1_basic_propagation();
    experiment_2_damping_by_weight();
    experiment_3_oscillation_frequencies();
    experiment_4_bridge_rigs();
    experiment_5_reflection();
}

fn build_network() -> Rigging {
    let mut rigging = Rigging::new();

    // Create 20 rigs arranged in a structured network
    // Layout: roughly a 4x5 grid with some extra connections
    let labels = [
        "core-0", "core-1", "core-2", "core-3",
        "mid-4", "mid-5", "mid-6", "mid-7",
        "edge-8", "edge-9", "edge-10", "edge-11",
        "leaf-12", "leaf-13", "leaf-14", "leaf-15",
        "hub-16", "hub-17", "hub-18", "hub-19",
    ];

    for (i, label) in labels.iter().enumerate() {
        rigging.add_rig(Rig::new(i, Trit::Zero, label));
    }

    // Core ring (0-1-2-3-0) — all weight +1
    rigging.add_rope(Rope::new(0, 1, 1));
    rigging.add_rope(Rope::new(1, 2, 1));
    rigging.add_rope(Rope::new(2, 3, 1));
    rigging.add_rope(Rope::new(3, 0, 1));

    // Core to mid layer
    rigging.add_rope(Rope::new(0, 4, 1));
    rigging.add_rope(Rope::new(1, 5, 1));
    rigging.add_rope(Rope::new(2, 6, 1));
    rigging.add_rope(Rope::new(3, 7, 1));

    // Mid layer connections (some with different weights)
    rigging.add_rope(Rope::new(4, 5, 1));
    rigging.add_rope(Rope::new(5, 6, 1));
    rigging.add_rope(Rope::new(6, 7, 1));
    rigging.add_rope(Rope::new(7, 4, 1));

    // Mid to edge (mixed weights: -1, 0, +1)
    rigging.add_rope(Rope::new(4, 8, 1));    // positive
    rigging.add_rope(Rope::new(5, 9, -1));   // inverted
    rigging.add_rope(Rope::new(6, 10, 0));   // blocked
    rigging.add_rope(Rope::new(7, 11, 1));   // positive

    // Edge to leaf
    rigging.add_rope(Rope::new(8, 12, 1));
    rigging.add_rope(Rope::new(9, 13, 1));
    rigging.add_rope(Rope::new(10, 14, 1));
    rigging.add_rope(Rope::new(11, 15, 1));

    // Hub rigs — highly connected bridge nodes
    rigging.add_rope(Rope::new(16, 0, 1));
    rigging.add_rope(Rope::new(16, 4, 1));
    rigging.add_rope(Rope::new(16, 8, 1));
    rigging.add_rope(Rope::new(17, 1, 1));
    rigging.add_rope(Rope::new(17, 5, 1));
    rigging.add_rope(Rope::new(17, 9, 1));
    rigging.add_rope(Rope::new(18, 2, 1));
    rigging.add_rope(Rope::new(18, 6, 1));
    rigging.add_rope(Rope::new(18, 10, 1));
    rigging.add_rope(Rope::new(19, 3, 1));
    rigging.add_rope(Rope::new(19, 7, 1));
    rigging.add_rope(Rope::new(19, 11, 1));

    // Some cross-connections for interesting dynamics
    rigging.add_rope(Rope::new(12, 16, 1));
    rigging.add_rope(Rope::new(13, 17, -1));
    rigging.add_rope(Rope::new(14, 18, 0));
    rigging.add_rope(Rope::new(15, 19, 1));

    rigging
}

/// Compute network-wide "energy" as sum of absolute values.
fn network_energy(rigging: &Rigging) -> (i32, i32) {
    let mut gamma: i32 = 0; // sum of values
    let mut h: i32 = 0;     // sum of absolute values (Hamiltonian analog)
    for i in 0..20 {
        if let Some(rig) = rigging.get_rig(i) {
            let v = rig.value.value() as i32;
            gamma += v;
            h += v.abs();
        }
    }
    (gamma, h)
}

fn print_network_state(rigging: &Rigging, label: &str) {
    let mut vals = Vec::new();
    for i in 0..20 {
        if let Some(rig) = rigging.get_rig(i) {
            vals.push(format!("{}={}", rig.label, match rig.value {
                Trit::Neg => "-1",
                Trit::Zero => " 0",
                Trit::Pos => "+1",
            }));
        }
    }
    println!("  [{}] {}", label, vals.join(", "));
}

fn experiment_1_basic_propagation() {
    println!("\n=== EXPERIMENT 1: Basic Propagation Distance ===\n");

    let mut rigging = build_network();

    // Shake each core rig and measure propagation
    for origin in 0..4 {
        let (g0, h0) = network_energy(&rigging);
        let traces = rigging.set_and_propagate(origin, Trit::Pos);
        let (g1, h1) = network_energy(&rigging);

        let max_step = traces.iter().map(|t| t.step).max().unwrap_or(0);
        let unique_targets: std::collections::HashSet<usize> = traces.iter().map(|t| t.to_id).collect();
        let conservation_delta = (g1 + h1) - (g0 + h0);

        println!("Origin {} ({}): reached {} rigs in {} steps, conservation delta={}, violated={}",
            origin, rigging.get_rig(origin).unwrap().label,
            unique_targets.len(), max_step, conservation_delta,
            conservation_delta != 0);

        for trace in &traces {
            let reached_val = rigging.get_rig(trace.to_id).map(|r| r.value.value()).unwrap_or(0);
            print!("  step {} → rig {} (val={}), transmitted={}",
                trace.step, trace.to_id, reached_val, trace.value_transmitted.value());
            println!();
        }

        // Reset
        for i in 0..20 {
            if let Some(rig) = rigging.get_rig_mut(i) {
                rig.set(Trit::Zero);
            }
        }
    }
}

fn experiment_2_damping_by_weight() {
    println!("\n=== EXPERIMENT 2: Damping by Connection Weight ===\n");

    // Three simple chains: weight -1, 0, +1
    for weight in [-1i8, 0, 1] {
        println!("--- Chain with weight {} ---", weight);
        let mut rigging = Rigging::new();
        for i in 0..10 {
            rigging.add_rig(Rig::new(i, Trit::Zero, &format!("chain-{}", i)));
        }
        for i in 0..9 {
            rigging.add_rope(Rope::new(i, i + 1, weight));
        }

        let traces = rigging.set_and_propagate(0, Trit::Pos);

        let max_step = traces.iter().map(|t| t.step).max().unwrap_or(0);
        let final_vals: Vec<String> = (0..10).map(|i| {
            rigging.get_rig(i).map(|r| format!("{}", r.value.value())).unwrap_or("?".to_string())
        }).collect();

        println!("  Propagation steps: {}", max_step);
        println!("  Values: [{}]", final_vals.join(", "));

        // Check: for weight 0, everything after first should be 0
        // For weight +1, all should be +1
        // For weight -1, alternating: +1, -1, +1, -1...
        let damping = if weight == 0 {
            "IMMEDIATE — all values blocked"
        } else if weight == 1 {
            "NONE — full propagation"
        } else {
            "SIGN-FLIP — values alternate"
        };
        println!("  Damping: {}", damping);
        println!();
    }
}

fn experiment_3_oscillation_frequencies() {
    println!("\n=== EXPERIMENT 3: Oscillation Frequencies ===\n");

    for freq in [1usize, 3, 5, 10] {
        println!("--- Frequency: every {} ticks ---", freq);

        let mut rigging = build_network();

        // Build oscillation pattern: shake at frequency over 30 ticks
        let total_ticks = 30;
        let mut pattern = Vec::new();
        for t in 0..total_ticks {
            if t % freq == 0 {
                // Alternate between Pos and Neg
                if (t / freq) % 2 == 0 {
                    pattern.push(Trit::Pos);
                } else {
                    pattern.push(Trit::Neg);
                }
            } else {
                pattern.push(Trit::Zero);
            }
        }

        let shake = RiggingShake::new(0, pattern.clone());
        let all_traces = rigging.shake(&shake);

        // Analyze final state
        let mut nonzero = 0;
        let mut pos_count = 0;
        let mut neg_count = 0;
        for i in 0..20 {
            if let Some(rig) = rigging.get_rig(i) {
                match rig.value {
                    Trit::Pos => { nonzero += 1; pos_count += 1; }
                    Trit::Neg => { nonzero += 1; neg_count += 1; }
                    Trit::Zero => {}
                }
            }
        }

        // Count total ripple traces per shake step
        let total_traces: usize = all_traces.iter().map(|t| t.len()).sum();
        let avg_traces = if all_traces.is_empty() { 0.0 } else { total_traces as f64 / all_traces.len() as f64 };

        // Track which rigs were affected at all
        let mut affected_rigs = std::collections::HashSet::new();
        for traces in &all_traces {
            for t in traces {
                affected_rigs.insert(t.to_id);
            }
        }

        println!("  Total ticks: {}, Shakes: {}", total_ticks, pattern.iter().filter(|&&t| t != Trit::Zero).count());
        println!("  Final nonzero rigs: {}/20 ({} pos, {} neg)", nonzero, pos_count, neg_count);
        println!("  Total ripple traces: {}, avg per shake: {:.1}", total_traces, avg_traces);
        println!("  Unique rigs affected: {}/20", affected_rigs.len());
        print_network_state(&rigging, &format!("freq-{}", freq));

        // Check for resonance: measure "energy" at each step
        let mut max_energy = 0;
        let mut energy_series = Vec::new();
        // Re-run to track energy over time
        let mut rigging2 = build_network();
        for trit in &pattern {
            rigging2.set_and_propagate(0, *trit);
            let (g, h) = network_energy(&rigging2);
            let energy = g.abs() + h;
            max_energy = max_energy.max(energy);
            energy_series.push(energy);
        }
        println!("  Energy trajectory: {:?}", energy_series);
        println!("  Peak energy: {}", max_energy);
        println!();
    }
}

fn experiment_4_bridge_rigs() {
    println!("\n=== EXPERIMENT 4: Bridge Rig Detection ===\n");

    // Shake each rig individually and measure network impact
    let mut results: Vec<(usize, String, usize, i32)> = Vec::new();

    for origin in 0..20 {
        let mut rigging = build_network();
        let traces = rigging.set_and_propagate(origin, Trit::Pos);

        let unique_targets: std::collections::HashSet<usize> = traces.iter().map(|t| t.to_id).collect();
        let label = rigging.get_rig(origin).unwrap().label.clone();
        let (g, h) = network_energy(&rigging);
        let total_energy = g.abs() + h;

        results.push((origin, label, unique_targets.len(), total_energy));
    }

    // Sort by impact (number of rigs affected)
    results.sort_by(|a, b| b.2.cmp(&a.2));

    println!("Rigs ranked by propagation reach (shaking with +1):");
    println!("{:<4} {:<10} {:<10} {:<10}", "ID", "Label", "Reached", "Energy");
    println!("{}", "-".repeat(40));
    for (id, label, reached, energy) in &results {
        let bridge_marker = if *reached >= 10 { " ← BRIDGE" } else { "" };
        println!("{:<4} {:<10} {:<10} {:<10}{}", id, label, reached, energy, bridge_marker);
    }

    // Identify bridges
    let bridges: Vec<_> = results.iter().filter(|(_, _, r, _)| *r >= 10).collect();
    println!("\nBridge rigs (affect ≥10 others): {} found", bridges.len());
    for (id, label, reached, energy) in &bridges {
        println!("  Rig {} ({}) reaches {} others, energy={}", id, label, reached, energy);
    }
}

fn experiment_5_reflection() {
    println!("\n=== EXPERIMENT 5: Boundary Reflection ===\n");

    // Build a linear chain and check if values "reflect" back
    let mut rigging = Rigging::new();
    for i in 0..8 {
        rigging.add_rig(Rig::new(i, Trit::Zero, &format!("linear-{}", i)));
    }
    // Forward chain
    for i in 0..7 {
        rigging.add_rope(Rope::new(i, i + 1, 1));
    }
    // Back link from end to create potential reflection
    rigging.add_rope(Rope::new(7, 6, 1));

    println!("Linear chain 0→1→2→3→4→5→6→7 with back-link 7→6:");
    let traces = rigging.set_and_propagate(0, Trit::Pos);
    for t in &traces {
        println!("  step {} : rig {} → rig {} (val={})", t.step, t.from_id, t.to_id, t.value_transmitted.value());
    }
    print_network_state(&rigging, "after propagation");

    // Now try with a pulley at the back-link (like a mirror)
    let mut rigging2 = Rigging::new();
    for i in 0..8 {
        rigging2.add_rig(Rig::new(i, Trit::Zero, &format!("mirror-{}", i)));
    }
    for i in 0..7 {
        rigging2.add_rope(Rope::new(i, i + 1, 1));
    }
    let back_rope_idx = rigging2.add_rope(Rope::new(7, 6, 1));
    rigging2.add_pulley(Pulley::new(back_rope_idx, true)); // inverting pulley = mirror

    println!("\nWith inverting pulley (mirror) at boundary:");
    let traces2 = rigging2.set_and_propagate(0, Trit::Pos);
    for t in &traces2 {
        println!("  step {} : rig {} → rig {} (val={})", t.step, t.from_id, t.to_id, t.value_transmitted.value());
    }
    print_network_state(&rigging2, "after mirror propagation");

    // Conservation check across experiments
    println!("\n=== CONSERVATION ANALYSIS ===\n");

    let mut rigging3 = build_network();
    let (g0, h0) = network_energy(&rigging3);
    println!("Initial state: gamma={}, H={}, gamma+H={}", g0, h0, g0 + h0);

    // Apply a shake pattern
    let pattern = vec![
        Trit::Pos, Trit::Neg, Trit::Pos, Trit::Neg, Trit::Pos,
        Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero,
    ];
    let shake = RiggingShake::new(0, pattern);

    let mut violations = Vec::new();
    for (tick, trit) in shake.pattern.iter().enumerate() {
        let (g_before, h_before) = network_energy(&rigging3);
        rigging3.set_and_propagate(shake.origin_id, *trit);
        let (g_after, h_after) = network_energy(&rigging3);

        let delta_gamma = g_after - g_before;
        let delta_h = h_after - h_before;
        let input_val = trit.value() as i32;

        // Conservation violation: (gamma+H) change != input value
        let total_delta = delta_gamma + delta_h;
        let violation = total_delta != input_val;

        if violation {
            violations.push((tick, input_val, total_delta, delta_gamma, delta_h));
        }

        println!("Tick {}: input={}, Δγ={}, ΔH={}, Δ(γ+H)={}, {}",
            tick, input_val, delta_gamma, delta_h, total_delta,
            if violation { "VIOLATION" } else { "ok" });
    }

    println!("\nConservation violations: {}/{} ticks", violations.len(), shake.pattern.len());
    for (tick, input, total, dg, dh) in &violations {
        println!("  Tick {}: input={}, Δ(γ+H)={}, (Δγ={}, ΔH={})", tick, input, total, dg, dh);
    }
}

//! Experiment 2: Symbiont Impact on Conservation
//!
//! Runs an ecosystem simulation WITHOUT symbionts for 500 ticks,
//! then introduces mutualistic symbiont pairs for 500 more ticks.
//! Compares conservation compliance and species fitness.

use ternary_cell::{CellGrid, Tissue};
use strategy_ecology::{LotkaVolterra, Population, Species as StratSpecies};
use ternary_ecosystem::{
    CarryingCapacity, Ecosystem, FoodWeb, Niche, Ternary,
    Species as EcoSpecies,
};
use ternary_symbiont::{
    Agent, SymbiontPair, SymbiosisDetector, compute_compatibility,
};

fn main() {
    println!("=== Symbiont Impact Experiment ===\n");

    let grid_size = 20;
    let species_names = ["Explorer", "Diplomat", "Marksman", "Climber", "Prospector"];
    let species_values: [i8; 5] = [1, 0, -1, 1, -1];

    // Create agents for each species (for symbiont pairing)
    let mut species_agents: Vec<Vec<Agent>> = (0..5).map(|idx| {
        let base_trait = species_values[idx] as f64;
        (0..10).map(|j| {
            Agent::with_traits(
                &format!("{}_{}", species_names[idx], j),
                vec![base_trait, (j as f64 % 3.0 - 1.0), base_trait * 0.5],
            )
        }).collect()
    }).collect();

    // Phase 1: No symbionts (500 ticks)
    println!("--- Phase 1: No Symbionts (ticks 0-499) ---");
    println!("tick,alive,gamma,entropy,gamma_plus_H,total_fitness,phase");

    let mut tissue = Tissue::new(grid_size, grid_size);
    for y in 0..grid_size {
        for x in 0..grid_size {
            let species_idx = ((x * 5) / grid_size + (y * 5) / grid_size) % 5;
            tissue.grid.place(x, y, species_values[species_idx]);
        }
    }

    let mut no_symb_gamma_h: Vec<(usize, f64)> = Vec::new();
    let mut no_symb_fitness: Vec<(usize, f64)> = Vec::new();
    let mut no_symb_alive: Vec<(usize, usize)> = Vec::new();
    let mut no_symb_species: Vec<(usize, [usize; 5])> = Vec::new();

    for tick in 0..500 {
        let alive = tissue.grid.tick_all();
        let (gamma, entropy) = compute_gamma_entropy(&tissue.grid, alive);
        let gamma_h = gamma + entropy;
        let total_fitness: f64 = tissue.grid.cells.iter()
            .filter_map(|c| c.as_ref().map(|c| c.energy as f64))
            .sum();

        no_symb_gamma_h.push((tick, gamma_h));
        no_symb_fitness.push((tick, total_fitness));
        no_symb_alive.push((tick, alive));
        no_symb_species.push((tick, count_species(&tissue, grid_size)));

        if tick % 50 == 0 || tick < 5 {
            println!("{},{},{:.4},{:.4},{:.4},{:.0},no_symbiont",
                tick, alive, gamma, entropy, gamma_h, total_fitness);
        }

        if alive == 0 {
            println!("All cells dead at tick {}", tick);
            break;
        }
    }

    // Save pre-symbiont state
    let pre_symb_state = tissue.grid.clone();
    let pre_symb_agents = species_agents.clone();

    // Phase 2: Introduce symbiont pairs (500 more ticks)
    println!("\n--- Phase 2: With Symbionts (ticks 500-999) ---");
    println!("tick,alive,gamma,entropy,gamma_plus_H,total_fitness,phase,symbiont_pairs");

    // Reset tissue for fair comparison (start from same initial state)
    let mut tissue_symb = pre_symb_state.clone();
    let mut agents_symb = pre_symb_agents.clone();

    // Create symbiont pairs between species
    let mut symbiont_pairs: Vec<SymbiontPair> = Vec::new();

    // Pair Explorer with Diplomat (complementary strategies)
    // Pair Marksman with Climber (complementary strategies)
    // Pair Prospector with Explorer (cross-niche)
    let pairings = [(0, 1), (2, 3), (4, 0), (1, 2), (3, 4)];

    for &(a_idx, b_idx) in &pairings {
        let count = agents_symb[a_idx].len().min(agents_symb[b_idx].len());
        for i in 0..count.min(3) {
            let a = agents_symb[a_idx][i].clone();
            let b = agents_symb[b_idx][i].clone();
            let compat = compute_compatibility(&a, &b);
            if compat.score > 0.0 {
                let mut pair = SymbiontPair::new(a, b);
                pair.interact(); // Initial bonding
                symbiont_pairs.push(pair);
            }
        }
    }

    println!("Created {} symbiont pairs", symbiont_pairs.len());

    let mut with_symb_gamma_h: Vec<(usize, f64)> = Vec::new();
    let mut with_symb_fitness: Vec<(usize, f64)> = Vec::new();
    let mut with_symb_alive: Vec<(usize, usize)> = Vec::new();
    let mut with_symb_species: Vec<(usize, [usize; 5])> = Vec::new();

    for tick in 500..1000 {
        // Symbiont interactions boost fitness
        let mut symb_gain = 0.0;
        for pair in &mut symbiont_pairs {
            symb_gain += pair.interact();
        }

        // Apply symbiont fitness as energy bonus to cells
        // Distribute symbiont gains across the grid
        let alive_cells: Vec<(usize, usize)> = (0..grid_size)
            .flat_map(|y| (0..grid_size).map(move |x| (x, y)))
            .filter(|&(x, y)| {
                tissue_symb.get(x, y).map_or(false, |c| c.is_alive())
            })
            .collect();

        if !alive_cells.is_empty() {
            let energy_bonus = (symb_gain / alive_cells.len() as f64 * 0.5) as i32;
            for &(x, y) in &alive_cells {
                if let Some(cell) = tissue_symb.get_mut(x, y) {
                    cell.energy = (cell.energy + energy_bonus).min(20);
                }
            }
        }

        let alive = tissue_symb.tick_all();
        let (gamma, entropy) = compute_gamma_entropy(&tissue_symb, alive);
        let gamma_h = gamma + entropy;
        let total_fitness: f64 = tissue_symb.cells.iter()
            .filter_map(|c| c.as_ref().map(|c| c.energy as f64))
            .sum();

        with_symb_gamma_h.push((tick, gamma_h));
        with_symb_fitness.push((tick, total_fitness));
        with_symb_alive.push((tick, alive));
        with_symb_species.push((tick, count_species(&Tissue { grid: tissue_symb.clone() }, grid_size)));

        if tick % 50 == 0 || tick < 505 {
            println!("{},{},{:.4},{:.4},{:.4},{:.0},symbiont,{}",
                tick, alive, gamma, entropy, gamma_h, total_fitness, symbiont_pairs.len());
        }

        if alive == 0 {
            println!("All cells dead at tick {}", tick);
            break;
        }
    }

    // Phase 3: Also run no-symbiont control for same period (500-999) from same pre-state
    println!("\n--- Phase 3: Control (no symbionts, ticks 500-999) ---");
    let mut tissue_control = pre_symb_state.clone();
    let mut ctrl_gamma_h: Vec<(usize, f64)> = Vec::new();
    let mut ctrl_fitness: Vec<(usize, f64)> = Vec::new();
    let mut ctrl_alive: Vec<(usize, usize)> = Vec::new();

    for tick in 500..1000 {
        let alive = tissue_control.tick_all();
        let (gamma, entropy) = compute_gamma_entropy(&tissue_control, alive);
        let gamma_h = gamma + entropy;
        let total_fitness: f64 = tissue_control.cells.iter()
            .filter_map(|c| c.as_ref().map(|c| c.energy as f64))
            .sum();

        ctrl_gamma_h.push((tick, gamma_h));
        ctrl_fitness.push((tick, total_fitness));
        ctrl_alive.push((tick, alive));

        if tick % 50 == 0 || tick < 505 {
            println!("{},{},{:.4},{:.4},{:.4},{:.0},control",
                tick, alive, gamma, entropy, gamma_h, total_fitness);
        }

        if alive == 0 {
            break;
        }
    }

    // --- Comparative Analysis ---
    println!("\n=== COMPARATIVE ANALYSIS ===");

    // Gamma+H comparison
    println!("\n--- Gamma+H Conservation ---");
    let ns_vals: Vec<f64> = no_symb_gamma_h.iter().map(|&(_, v)| v).collect();
    let ws_vals: Vec<f64> = with_symb_gamma_h.iter().map(|&(_, v)| v).collect();
    let ct_vals: Vec<f64> = ctrl_gamma_h.iter().map(|&(_, v)| v).collect();

    println!("No-symbiont phase (0-499):");
    print_brief_stats(&ns_vals);
    println!("\nWith-symbiont phase (500-999):");
    print_brief_stats(&ws_vals);
    println!("\nControl (no symbionts, 500-999):");
    print_brief_stats(&ct_vals);

    // Fitness comparison
    println!("\n--- Total Fitness ---");
    let ns_fit: Vec<f64> = no_symb_fitness.iter().map(|&(_, v)| v).collect();
    let ws_fit: Vec<f64> = with_symb_fitness.iter().map(|&(_, v)| v).collect();
    let ct_fit: Vec<f64> = ctrl_fitness.iter().map(|&(_, v)| v).collect();

    println!("No-symbiont mean fitness:  {:.1}", mean(&ns_fit));
    println!("With-symbiont mean fitness: {:.1}", mean(&ws_fit));
    println!("Control mean fitness:       {:.1}", mean(&ct_fit));

    // Survival comparison
    println!("\n--- Cell Survival ---");
    let ns_surv: Vec<usize> = no_symb_alive.iter().map(|&(_, v)| v).collect();
    let ws_surv: Vec<usize> = with_symb_alive.iter().map(|&(_, v)| v).collect();
    let ct_surv: Vec<usize> = ctrl_alive.iter().map(|&(_, v)| v).collect();

    println!("No-symbiont final alive:  {}", ns_surv.last().unwrap_or(&0));
    println!("With-symbiont final alive: {}", ws_surv.last().unwrap_or(&0));
    println!("Control final alive:       {}", ct_surv.last().unwrap_or(&0));

    // Conservation stability comparison
    println!("\n--- Conservation Stability (lower CV = more stable) ---");
    let ns_cv = cv(&ns_vals);
    let ws_cv = cv(&ws_vals);
    let ct_cv = cv(&ct_vals);
    println!("No-symbiont CV:  {:.4}", ns_cv);
    println!("With-symbiont CV: {:.4}", ws_cv);
    println!("Control CV:       {:.4}", ct_cv);

    let improvement = if ns_cv > 0.0 {
        ((ns_cv - ws_cv) / ns_cv) * 100.0
    } else {
        0.0
    };
    println!("Conservation improvement with symbiosis: {:.1}%", improvement);

    // Species-level analysis
    println!("\n--- Species Distribution (last measurement) ---");
    if let Some((_, last_ns)) = no_symb_species.last() {
        println!("No-symbiont: {:?}", last_ns);
    }
    if let Some((_, last_ws)) = with_symb_species.last() {
        println!("With-symbiont: {:?}", last_ws);
    }
    if let Some((_, last_ct)) = ctrl_alive.last() {
        println!("Control alive: {}", last_ct);
    }

    // Symbiont pair analysis
    println!("\n--- Symbiont Pair Analysis ---");
    for (i, pair) in symbiont_pairs.iter().enumerate() {
        println!("Pair {}: {} + {} | fitness=({:.2}, {:.2}) | bond={:.2} | healthy={}",
            i, pair.agent_a.id, pair.agent_b.id,
            pair.agent_a.fitness, pair.agent_b.fitness,
            pair.bond_strength, pair.is_healthy());
    }

    // Per-species benefit analysis
    println!("\n--- Per-Species Benefit from Symbiosis ---");
    let no_symb_last = no_symb_species.last().map(|&(_, ref s)| *s).unwrap_or([0; 5]);
    let with_symb_last = with_symb_species.last().map(|&(_, ref s)| *s).unwrap_or([0; 5]);

    for i in 0..5 {
        let delta = with_symb_last[i] as i64 - no_symb_last[i] as i64;
        let pct = if no_symb_last[i] > 0 {
            delta as f64 / no_symb_last[i] as f64 * 100.0
        } else {
            0.0
        };
        println!("  {}: no_symb={} with_symb={} delta={} ({:.1}%)",
            species_names[i], no_symb_last[i], with_symb_last[i], delta, pct);
    }
}

fn compute_gamma_entropy(grid: &CellGrid, alive: u32) -> (f64, f64) {
    let total = alive as f64;
    if total == 0.0 {
        return (0.0, 0.0);
    }

    let (pos, zero, neg) = grid.tissue_balance();

    let p_pos = pos as f64 / total;
    let p_neg = neg as f64 / total;

    let gamma = 1.0 - (p_pos - p_neg).abs();

    let mut entropy = 0.0;
    for &count in &[pos, zero, neg] {
        if count > 0 {
            let p = count as f64 / total;
            entropy -= p * p.log2();
        }
    }

    (gamma, entropy)
}

fn count_species(tissue: &Tissue, grid_size: usize) -> [usize; 5] {
    let mut counts = [0usize; 5];
    for y in 0..grid_size {
        for x in 0..grid_size {
            if let Some(cell) = tissue.grid.get(x, y) {
                if cell.is_alive() {
                    let species_idx = ((x * 5) / grid_size + (y * 5) / grid_size) % 5;
                    counts[species_idx] += 1;
                }
            }
        }
    }
    counts
}

fn mean(values: &[f64]) -> f64 {
    if values.is_empty() { return 0.0; }
    values.iter().sum::<f64>() / values.len() as f64
}

fn cv(values: &[f64]) -> f64 {
    let m = mean(values);
    if m.abs() < 1e-10 { return 0.0; }
    let variance = values.iter().map(|v| (v - m).powi(2)).sum::<f64>() / values.len() as f64;
    variance.sqrt() / m.abs()
}

fn print_brief_stats(values: &[f64]) {
    let m = mean(values);
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let variance = values.iter().map(|v| (v - m).powi(2)).sum::<f64>() / values.len() as f64;
    println!("  Mean={:.4} StdDev={:.4} Range=[{:.4}, {:.4}] CV={:.4}",
        m, variance.sqrt(), min, max, cv(values));
}

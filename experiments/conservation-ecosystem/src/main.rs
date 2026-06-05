//! Experiment 1: Conservation Law Across Ecosystem Levels
//!
//! Combines CellGrid (ternary-cell) with Population/LotkaVolterra (strategy-ecology)
//! and Ecosystem (ternary-ecosystem) to test whether conservation compliance
//! holds across spatial ecosystem simulation.

use ternary_cell::{CellGrid, TernaryMessenger, Tissue};
use strategy_ecology::{
    LotkaVolterra, Population, Species as StratSpecies,
};
use ternary_ecosystem::{
    CarryingCapacity, Ecosystem, FoodWeb, Niche, Ternary,
    Species as EcoSpecies, SuccessionStage,
};

fn main() {
    println!("=== Conservation-Ecosystem Experiment ===\n");

    // --- Phase 1: CellGrid + Strategy Ecology ---
    println!("--- Phase 1: CellGrid with Strategy Species ---");

    let grid_size = 20; // 20x20 grid = 400 cells
    let mut tissue = Tissue::new(grid_size, grid_size);

    // Populate with 5 strategy species in clusters
    // Species: Explorer(0), Diplomat(1), Marksman(2), Climber(3), Prospector(4)
    let species_names = ["Explorer", "Diplomat", "Marksman", "Climber", "Prospector"];
    let species_values: [i8; 5] = [1, 0, -1, 1, -1]; // ternary value per species

    // Place cells in quadrant-like clusters
    for y in 0..grid_size {
        for x in 0..grid_size {
            let species_idx = ((x * 5) / grid_size + (y * 5) / grid_size) % 5;
            tissue.grid.place(x, y, species_values[species_idx]);
        }
    }

    println!("Initial alive cells: {}", tissue.grid.alive_count());

    // CSV header
    println!("tick,alive,gamma,entropy,explorer,diplomat,marksman,climber,prospector,total_fitness");

    let mut prev_balance: Option<f64> = None;
    let mut gamma_h_values: Vec<f64> = Vec::new();

    for tick in 0..1000 {
        let alive = tissue.grid.tick_all();

        // Count species distribution by ternary value (proxy for species)
        let (pos, zero, neg) = tissue.grid.tissue_balance();

        // Compute gamma: conservation ratio
        // Gamma = balance of ternary values, normalized
        let total_cells = alive as f64;
        let gamma = if total_cells > 0.0 {
            let p_pos = pos as f64 / total_cells;
            let p_neg = neg as f64 / total_cells;
            // Gamma measures how balanced the ternary distribution is
            // Gamma = 1 - |p_pos - p_neg| (1 = perfectly balanced)
            1.0 - (p_pos - p_neg).abs()
        } else {
            0.0
        };

        // Compute Shannon entropy
        let entropy = if total_cells > 0.0 {
            let mut h = 0.0;
            for &count in &[pos, zero, neg] {
                if count > 0 {
                    let p = count as f64 / total_cells;
                    h -= p * p.log2();
                }
            }
            h
        } else {
            0.0
        };

        let gamma_h = gamma + entropy;
        gamma_h_values.push(gamma_h);

        // Approximate fitness as total energy
        let total_fitness: i32 = tissue.grid.cells.iter()
            .filter_map(|c| c.as_ref().map(|c| c.energy))
            .sum();

        // Species distribution (by grid quadrant)
        let species_counts = count_species(&tissue, grid_size);

        if tick % 50 == 0 || tick < 10 || tick > 990 {
            println!("{},{:.3},{:.3},{},{},{},{},{},{},{}",
                tick, alive, gamma, entropy,
                species_counts[0], species_counts[1], species_counts[2],
                species_counts[3], species_counts[4], total_fitness);
        }

        if alive == 0 {
            println!("All cells dead at tick {}", tick);
            break;
        }

        prev_balance = Some(gamma_h);
    }

    // --- Phase 2: Lotka-Volterra population dynamics ---
    println!("\n--- Phase 2: Lotka-Volterra Population Dynamics ---");

    let lv = LotkaVolterra::default_interaction_matrix();
    let mut pop = Population::balanced(1000.0);

    println!("tick,pop_total,entropy,normalized_entropy,simpson,richness,gamma,gamma_plus_H");
    let mut lv_gamma_h: Vec<f64> = Vec::new();

    for tick in 0..1000 {
        pop = lv.step(&pop, 0.01);

        let entropy = pop.shannon_entropy();
        let norm_entropy = pop.normalized_entropy();
        let simpson = pop.simpson_index();
        let richness = pop.species_richness();
        let total = pop.total();

        // Gamma for LV: balance of population across species
        let gamma = if total > 0.0 {
            let counts = pop.counts();
            let max_count = counts.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let min_count = counts.iter().cloned().fold(f64::INFINITY, f64::min);
            if max_count > 0.0 {
                1.0 - (max_count - min_count) / total
            } else {
                0.0
            }
        } else {
            0.0
        };

        let gamma_h = gamma + entropy;
        lv_gamma_h.push(gamma_h);

        if tick % 100 == 0 || tick < 5 {
            println!("{},{:.1},{:.4},{:.4},{:.4},{},{:.4},{:.4}",
                tick, total, entropy, norm_entropy, simpson, richness, gamma, gamma_h);
        }
    }

    // --- Phase 3: Full Ecosystem (ternary-ecosystem) ---
    println!("\n--- Phase 3: Full Ecosystem Simulation ---");

    let eco_species = vec![
        EcoSpecies::new("Explorer", 300, 15, 0, Ternary::Pos),
        EcoSpecies::new("Diplomat", 250, 12, 1, Ternary::Zero),
        EcoSpecies::new("Marksman", 200, 10, 2, Ternary::Neg),
        EcoSpecies::new("Climber", 150, 8, 1, Ternary::Pos),
        EcoSpecies::new("Prospector", 100, 6, 0, Ternary::Neg),
    ];

    let mut food_web = FoodWeb::new();
    // Trophic links: Climber eats Explorer, Diplomat eats Prospector, Marksman eats Diplomat
    food_web.add_link(3, 0, 30); // Climber eats Explorer
    food_web.add_link(1, 4, 25); // Diplomat eats Prospector
    food_web.add_link(2, 1, 20); // Marksman eats Diplomat

    let niches = vec![
        Niche::new("General", vec![Ternary::Pos, Ternary::Zero], 500),
        Niche::new("Specialized", vec![Ternary::Neg, Ternary::Neg], 300),
    ];

    let cc = CarryingCapacity::new(2000, vec![500, 300]);
    let mut ecosystem = Ecosystem::new(eco_species, food_web, niches, cc);

    println!("tick,total_pop,living,gamma,entropy,gamma_plus_H,stage");
    let mut eco_gamma_h: Vec<f64> = Vec::new();

    for tick in 0..1000 {
        ecosystem.tick();

        let total = ecosystem.total_population();
        let living = ecosystem.living_count();
        let stage = format!("{:?}", ecosystem.succession.current_stage);

        // Compute gamma and entropy from species populations
        let (gamma, entropy) = if total > 0 {
            let pops: Vec<f64> = ecosystem.species.iter()
                .map(|s| s.population as f64).collect();
            let max_p = pops.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let min_p = pops.iter().cloned().fold(f64::INFINITY, f64::min);
            let g = 1.0 - (max_p - min_p) / total;
            let mut h = 0.0;
            for &p in &pops {
                if p > 0.0 {
                    let prob = p / total;
                    h -= prob * prob.log2();
                }
            }
            (g, h)
        } else {
            (0.0, 0.0)
        };

        let gamma_h = gamma + entropy;
        eco_gamma_h.push(gamma_h);

        if tick % 100 == 0 || tick < 5 {
            println!("{},{},{},{:.4},{:.4},{:.4},{}",
                tick, total, living, gamma, entropy, gamma_h, stage);
        }
    }

    // --- Summary Statistics ---
    println!("\n=== SUMMARY STATISTICS ===");

    println!("\n--- CellGrid Conservation ---");
    print_stats("gamma+H", &gamma_h_values);

    println!("\n--- Lotka-Volterra Conservation ---");
    print_stats("gamma+H", &lv_gamma_h);

    println!("\n--- Ecosystem Conservation ---");
    print_stats("gamma+H", &eco_gamma_h);

    // Phase transition analysis
    println!("\n=== PHASE TRANSITION ANALYSIS ===");
    analyze_transitions(&gamma_h_values, "CellGrid");
    analyze_transitions(&lv_gamma_h, "Lotka-Volterra");
    analyze_transitions(&eco_gamma_h, "Ecosystem");

    // Critical density analysis
    println!("\n=== CRITICAL DENSITY ANALYSIS ===");
    println!("The conservation quantity gamma+H was tracked across all simulation phases.");
    println!("Stable conservation indicates the system maintains ternary balance.");
    println!("Breakdowns indicate phase transitions or ecosystem restructuring.");
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

fn print_stats(name: &str, values: &[f64]) {
    if values.is_empty() {
        println!("  No data");
        return;
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
    let stddev = variance.sqrt();

    let first_100_mean = values.iter().take(100).sum::<f64>() / values.len().min(100) as f64;
    let last_100_mean = values.iter().rev().take(100).sum::<f64>() / values.len().min(100) as f64;

    println!("  {} statistics:", name);
    println!("    Mean:     {:.4}", mean);
    println!("    StdDev:   {:.4}", stddev);
    println!("    Min:      {:.4} (tick {})", min, values.iter().position(|&v| v == min).unwrap_or(0));
    println!("    Max:      {:.4} (tick {})", max, values.iter().position(|&v| v == max).unwrap_or(0));
    println!("    Range:    {:.4}", max - min);
    println!("    First 100 mean: {:.4}", first_100_mean);
    println!("    Last 100 mean:  {:.4}", last_100_mean);
    println!("    Drift:    {:.4}", last_100_mean - first_100_mean);

    // Coefficient of variation (consistency measure)
    let cv = if mean.abs() > 1e-10 { stddev / mean.abs() } else { f64::NAN };
    println!("    CV:       {:.4}", cv);
}

fn analyze_transitions(values: &[f64], label: &str) {
    if values.len() < 10 {
        return;
    }

    // Detect phase transitions: points where gamma+H changes by more than 2*stddev
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
    let stddev = variance.sqrt();

    let threshold = 2.0 * stddev;
    let mut transitions = Vec::new();

    for i in 1..values.len() {
        let delta = (values[i] - values[i - 1]).abs();
        if delta > threshold {
            transitions.push((i, delta));
        }
    }

    println!("{}: {} phase transitions detected (threshold={:.4}):", label, transitions.len(), threshold);
    for &(tick, delta) in transitions.iter().take(10) {
        println!("  Tick {}: delta={:.4}", tick, delta);
    }
    if transitions.len() > 10 {
        println!("  ... and {} more", transitions.len() - 10);
    }
}

//! Experiment: Does Conservation Survive Evolution?
//!
//! Combines CellGrid (ternary-cell) with Species (strategy-ecology) in an
//! evolutionary simulation. Tracks gamma (conservation ratio), Shannon entropy,
//! and species distribution over 2000 ticks to test whether gamma+H is conserved.

use rand::Rng;
use std::collections::HashMap;

use ternary_cell::{CellGrid, TernaryCell, TernaryMessenger};
use strategy_ecology::Species as StratSpecies;

/// A cell in the ecosystem that carries a strategy species tag.
#[derive(Debug, Clone)]
struct EcoCell {
    cell: TernaryCell,
    species: usize, // 0-4 index into StratSpecies::all()
    fitness: f64,
}

/// The evolutionary ecosystem: a grid of species-tagged ternary cells.
struct EcoGrid {
    width: usize,
    height: usize,
    grid: Vec<Option<EcoCell>>,
    next_id: u64,
    tick: u64,
    mutation_rate: f64,
    death_threshold: f64,
    reproduction_threshold: f64,
}

impl EcoGrid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![None; width * height],
            next_id: 0,
            tick: 0,
            mutation_rate: 0.02,
            death_threshold: 0.15,
            reproduction_threshold: 0.75,
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn populate_random(&mut self) {
        let mut rng = rand::thread_rng();
        let species = StratSpecies::all();
        for y in 0..self.height {
            for x in 0..self.width {
                let sp_idx = rng.gen_range(0..5);
                let ternary_val = match sp_idx {
                    0 => 1,  // Explorer -> Signal
                    1 => 0,  // Diplomat -> Silence
                    2 => -1, // Marksman -> Suppress
                    3 => 1,  // Climber -> Signal
                    4 => -1, // Prospector -> Suppress
                    _ => 0,
                };
                let id = self.next_id;
                self.next_id += 1;
                let mut cell = TernaryCell::with_value(id, ternary_val);
                cell.energy = 10;
                self.grid[self.idx(x, y)] = Some(EcoCell {
                    cell,
                    species: sp_idx,
                    fitness: 0.5,
                });
            }
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        if x > 0 { result.push((x - 1, y)); }
        if x + 1 < self.width { result.push((x + 1, y)); }
        if y > 0 { result.push((x, y - 1)); }
        if y + 1 < self.height { result.push((x, y + 1)); }
        result
    }

    /// Compute fitness for each cell based on species interactions and energy.
    fn compute_fitness(&mut self) {
        let width = self.width;
        let height = self.height;

        // Collect species info for neighbor lookup
        let species_map: Vec<Option<usize>> = self.grid.iter().map(|c| c.as_ref().map(|c| c.species)).collect();

        for y in 0..height {
            for x in 0..width {
                let idx = self.idx(x, y);
                if let Some(ec) = &mut self.grid[idx] {
                    let my_species = ec.species;
                    let my_val = ec.cell.ternary_value;
                    let my_energy = ec.cell.energy;

                    let mut coop_bonus = 0.0_f64;
                    let mut compat_bonus = 0.0_f64;
                    let mut neighbor_count = 0;

                    for (nx, ny) in self.neighbors(x, y) {
                        let nidx = self.idx(nx, ny);
                        if let Some(Some(n_sp)) = species_map.get(nidx) {
                            neighbor_count += 1;
                            // Cooperation: same species nearby
                            if *n_sp == my_species {
                                coop_bonus += 0.15;
                            }
                            // Compatibility: different ternary values create balance
                            // Matching: Diplomats (0) coexist well with everyone
                            if my_species == 1 || *n_sp == 1 {
                                compat_bonus += 0.05;
                            }
                        }
                    }

                    // Energy-based survival fitness
                    let energy_fitness = (my_energy as f64 / 20.0).min(1.0);

                    // Species-specific base fitness
                    let base = match my_species {
                        0 => 0.4, // Explorer: moderate baseline
                        1 => 0.5, // Diplomat: high baseline (adapter)
                        2 => 0.3, // Marksman: low baseline, needs precision
                        3 => 0.45, // Climber: moderate
                        4 => 0.35, // Prospector: low baseline, needs space
                        _ => 0.4,
                    };

                    let fitness = base + energy_fitness * 0.3 + coop_bonus + compat_bonus;
                    ec.fitness = fitness.clamp(0.0, 1.0);
                }
            }
        }
    }

    /// Selection: remove low-fitness cells.
    fn selection(&mut self) -> usize {
        let mut rng = rand::thread_rng();
        let mut died = 0;
        for ec_opt in &mut self.grid {
            if let Some(ec) = ec_opt {
                if ec.fitness < self.death_threshold {
                    *ec_opt = None;
                    died += 1;
                }
            }
        }
        died
    }

    /// Reproduction: high-fitness cells spread to empty neighbors.
    fn reproduction(&mut self) -> usize {
        let width = self.width;
        let height = self.height;
        let mut rng = rand::thread_rng();
        let mut born = 0;

        // Collect reproduction candidates
        let mut candidates: Vec<(usize, usize, usize, usize)> = Vec::new(); // (x, y, nx, ny)
        for y in 0..height {
            for x in 0..width {
                let idx = self.idx(x, y);
                if let Some(ec) = &self.grid[idx] {
                    if ec.fitness > self.reproduction_threshold && ec.cell.energy >= 8 {
                        for (nx, ny) in self.neighbors(x, y) {
                            let nidx = self.idx(nx, ny);
                            if self.grid[nidx].is_none() {
                                candidates.push((x, y, nx, ny));
                            }
                        }
                    }
                }
            }
        }

        for (x, y, nx, ny) in candidates {
            let idx = self.idx(x, y);
            let nidx = self.idx(nx, ny);
            if self.grid[nidx].is_some() {
                continue; // already filled
            }
            if let Some(parent) = &mut self.grid[idx] {
                if parent.cell.energy < 5 {
                    continue;
                }
                // Halve parent energy
                parent.cell.energy /= 2;

                let id = self.next_id;
                self.next_id += 1;

                // Offspring inherits species (with possible mutation)
                let mut child_species = parent.species;
                if rng.gen::<f64>() < self.mutation_rate {
                    child_species = rng.gen_range(0..5);
                }

                let ternary_val = match child_species {
                    0 => 1, 1 => 0, 2 => -1, 3 => 1, 4 => -1, _ => 0,
                };

                let mut child_cell = TernaryCell::with_value(id, ternary_val);
                child_cell.energy = parent.cell.energy;
                child_cell.generation = parent.cell.generation + 1;

                self.grid[nidx] = Some(EcoCell {
                    cell: child_cell,
                    species: child_species,
                    fitness: 0.5,
                });
                born += 1;
            }
        }

        born
    }

    /// Mutation: randomly change species of some cells.
    fn mutation(&mut self) -> usize {
        let mut rng = rand::thread_rng();
        let mut mutated = 0;
        for ec_opt in &mut self.grid {
            if let Some(ec) = ec_opt {
                if rng.gen::<f64>() < self.mutation_rate {
                    ec.species = rng.gen_range(0..5);
                    ec.cell.ternary_value = match ec.species {
                        0 => 1, 1 => 0, 2 => -1, 3 => 1, 4 => -1, _ => 0,
                    };
                    mutated += 1;
                }
            }
        }
        mutated
    }

    /// Signal propagation and cell tick (energy dynamics).
    fn cell_dynamics(&mut self) {
        let width = self.width;
        let height = self.height;

        // Collect emissions
        let mut emissions: Vec<(usize, usize, TernaryMessenger)> = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let idx = self.idx(x, y);
                if let Some(ec) = &self.grid[idx] {
                    emissions.push((x, y, ec.cell.emit()));
                }
            }
        }

        // Deliver signals
        for (x, y, msg) in emissions {
            for (nx, ny) in self.neighbors(x, y) {
                let nidx = self.idx(nx, ny);
                if let Some(ec) = &mut self.grid[nidx] {
                    ec.cell.receive(msg);
                }
            }
        }

        // Tick all cells
        for ec_opt in &mut self.grid {
            if let Some(ec) = ec_opt {
                ec.cell.tick();
                if !ec.cell.is_alive() {
                    *ec_opt = None;
                }
            }
        }
    }

    /// Run one full evolutionary tick.
    fn evolve(&mut self) -> (usize, usize, usize) {
        self.cell_dynamics();
        self.compute_fitness();
        let died = self.selection();
        let born = self.reproduction();
        let mutated = self.mutation();

        // Energy regeneration for surviving cells
        for ec_opt in &mut self.grid {
            if let Some(ec) = ec_opt {
                ec.cell.energy = (ec.cell.energy + 1).min(20);
            }
        }

        self.tick += 1;
        (died, born, mutated)
    }

    /// Count alive cells.
    fn alive_count(&self) -> usize {
        self.grid.iter().filter(|c| c.is_some()).count()
    }

    /// Species distribution counts.
    fn species_counts(&self) -> [usize; 5] {
        let mut counts = [0usize; 5];
        for ec_opt in &self.grid {
            if let Some(ec) = ec_opt {
                counts[ec.species] += 1;
            }
        }
        counts
    }

    /// Ternary value distribution.
    fn ternary_balance(&self) -> (usize, usize, usize) {
        let mut pos = 0;
        let mut zero = 0;
        let mut neg = 0;
        for ec_opt in &self.grid {
            if let Some(ec) = ec_opt {
                match ec.cell.ternary_value {
                    1 => pos += 1,
                    0 => zero += 1,
                    -1 => neg += 1,
                    _ => {}
                }
            }
        }
        (pos, zero, neg)
    }

    /// Compute gamma: conservation ratio (balance of ternary values).
    /// Gamma = 1 - |f_pos - f_neg|, where f is fraction.
    fn gamma(&self) -> f64 {
        let alive = self.alive_count() as f64;
        if alive == 0.0 {
            return 0.0;
        }
        let (pos, _, neg) = self.ternary_balance();
        let f_pos = pos as f64 / alive;
        let f_neg = neg as f64 / alive;
        1.0 - (f_pos - f_neg).abs()
    }

    /// Shannon entropy over species distribution.
    fn shannon_entropy(&self) -> f64 {
        let alive = self.alive_count() as f64;
        if alive == 0.0 {
            return 0.0;
        }
        let counts = self.species_counts();
        let mut h = 0.0;
        for &c in &counts {
            if c > 0 {
                let p = c as f64 / alive;
                h -= p * p.log2();
            }
        }
        h
    }

    /// Total fitness.
    fn total_fitness(&self) -> f64 {
        self.grid.iter().filter_map(|c| c.as_ref().map(|c| c.fitness)).sum()
    }

    /// Number of living species.
    fn living_species(&self) -> usize {
        let counts = self.species_counts();
        counts.iter().filter(|&&c| c > 0).count()
    }
}

fn main() {
    println!("tick,alive,gamma,H,gamma_plus_H,sp0,sp1,sp2,sp3,sp4,total_fitness");

    let mut grid = EcoGrid::new(10, 10);
    grid.populate_random();

    let species_names = ["Explorer", "Diplomat", "Marksman", "Climber", "Prospector"];
    let total_ticks = 2000;

    // Track gamma+H for analysis
    let mut gamma_h_series: Vec<(usize, f64, usize)> = Vec::new(); // (tick, gamma+H, living_species)
    let mut species_alive_at: [Option<usize>; 5] = [None; 5]; // last tick species was alive
    let mut species_died_at: [Option<usize>; 5] = [None; 5]; // tick species died

    for tick in 0..total_ticks {
        let (died, born, mutated) = grid.evolve();

        let alive = grid.alive_count();
        let gamma = grid.gamma();
        let h = grid.shannon_entropy();
        let gamma_h = gamma + h;
        let counts = grid.species_counts();
        let total_fitness = grid.total_fitness();

        // Track species alive/dead
        let living_sp = grid.living_species();
        for (i, &c) in counts.iter().enumerate() {
            if c > 0 {
                species_alive_at[i] = Some(tick);
                species_died_at[i] = None;
            } else if species_died_at[i].is_none() && species_alive_at[i].is_some() {
                species_died_at[i] = Some(tick);
            }
        }

        gamma_h_series.push((tick, gamma_h, living_sp));

        // Print CSV for every tick
        println!("{},{},{:.4},{:.4},{:.4},{},{},{},{},{},{:.2}",
            tick, alive, gamma, h, gamma_h,
            counts[0], counts[1], counts[2], counts[3], counts[4],
            total_fitness);

        if alive == 0 {
            eprintln!("All cells dead at tick {}", tick);
            break;
        }
    }

    // === SUMMARY ===
    eprintln!("\n=== SUMMARY ===");

    // Mean and stddev of gamma+H
    if !gamma_h_series.is_empty() {
        let values: Vec<f64> = gamma_h_series.iter().map(|&(_, v, _)| v).collect();
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
        let stddev = variance.sqrt();
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        eprintln!("gamma+H statistics:");
        eprintln!("  Mean:   {:.4}", mean);
        eprintln!("  StdDev: {:.4}", stddev);
        eprintln!("  Min:    {:.4}", min);
        eprintln!("  Max:    {:.4}", max);
        eprintln!("  Range:  {:.4}", max - min);
        eprintln!("  CV:     {:.4}", stddev / mean.abs());

        // First/last 200 comparison
        let first_200: Vec<f64> = values.iter().take(200).copied().collect();
        let last_200: Vec<f64> = values.iter().rev().take(200).copied().collect();
        let first_mean = first_200.iter().sum::<f64>() / first_200.len() as f64;
        let last_mean = last_200.iter().sum::<f64>() / last_200.len() as f64;
        eprintln!("  First 200 mean: {:.4}", first_mean);
        eprintln!("  Last 200 mean:  {:.4}", last_mean);
        eprintln!("  Drift:          {:.4}", last_mean - first_mean);
    }

    // Species survival
    eprintln!("\nSpecies survival:");
    for (i, name) in species_names.iter().enumerate() {
        let counts = grid.species_counts();
        if counts[i] > 0 {
            eprintln!("  {} ({}) SURVIVED with {} individuals", i, name, counts[i]);
        } else if let Some(died_at) = species_died_at[i] {
            eprintln!("  {} ({}) EXTINCT at tick {}", i, name, died_at);
        }
    }

    // Phase transitions: sudden drops in species count
    eprintln!("\nPhase transitions (sudden drops in living species count):");
    let mut prev_sp = 5;
    let mut transitions: Vec<(usize, i32)> = Vec::new();
    for &(tick, _, living_sp) in &gamma_h_series {
        let delta = living_sp as i32 - prev_sp;
        if delta < 0 {
            transitions.push((tick, delta));
        }
        prev_sp = living_sp;
    }
    if transitions.is_empty() {
        eprintln!("  No species loss transitions detected");
    } else {
        for (tick, delta) in &transitions {
            eprintln!("  Tick {}: lost {} species", tick, -delta);
        }
    }

    // Correlation between gamma+H and species diversity
    eprintln!("\nCorrelation between gamma+H and species diversity:");
    if gamma_h_series.len() > 2 {
        let gh: Vec<f64> = gamma_h_series.iter().map(|&(_, v, _)| v).collect();
        let sp: Vec<f64> = gamma_h_series.iter().map(|&(_, _, s)| s as f64).collect();

        let gh_mean = gh.iter().sum::<f64>() / gh.len() as f64;
        let sp_mean = sp.iter().sum::<f64>() / sp.len() as f64;

        let cov: f64 = gh.iter().zip(sp.iter())
            .map(|(g, s)| (g - gh_mean) * (s - sp_mean))
            .sum::<f64>() / gh.len() as f64;

        let gh_std = (gh.iter().map(|g| (g - gh_mean).powi(2)).sum::<f64>() / gh.len() as f64).sqrt();
        let sp_std = (sp.iter().map(|s| (s - sp_mean).powi(2)).sum::<f64>() / sp.len() as f64).sqrt();

        let correlation = if gh_std > 0.0 && sp_std > 0.0 {
            cov / (gh_std * sp_std)
        } else {
            0.0
        };

        eprintln!("  Pearson r = {:.4}", correlation);
        eprintln!("  Interpretation: {}", if correlation > 0.5 {
            "Strong positive - higher gamma+H associated with more species"
        } else if correlation > 0.2 {
            "Moderate positive - some relationship between conservation and diversity"
        } else if correlation > -0.2 {
            "Weak/none - conservation and diversity are largely independent"
        } else {
            "Negative - surprising inverse relationship"
        });
    }
}

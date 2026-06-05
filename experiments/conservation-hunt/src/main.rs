// Conservation Hunt: What quantity is ACTUALLY conserved under ternary evolution?
// Tests 5 candidates after finding gamma+H fails (-25% drift)
// Candidates: total energy, info content, fitness-weighted entropy, Lyapunov function, population variance

fn main() {
    println!("tick,alive,gamma,H,gamma_plus_H,total_energy,info_content,fitness_entropy,pop_variance,species_1,species_2,species_3,species_4,species_5");

    // 10x10 grid, 5 species, 2000 ticks
    let grid_size = 100;
    let num_species = 5usize;
    let mut cells: Vec<i8> = Vec::with_capacity(grid_size);
    let mut species: Vec<usize> = Vec::with_capacity(grid_size);
    let mut fitness: Vec<f64> = Vec::with_capacity(grid_size);

    // PRNG
    let mut state: u64 = 12345;
    let mut rng = || -> f64 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((state >> 33) as f64) / (1u64 << 31) as f64
    };

    // Initialize: random ternary values, random species, fitness from value
    for _ in 0..grid_size {
        let v = match (rng() * 3.0) as usize { 0 => -1, 1 => 0, _ => 1 };
        cells.push(v);
        species.push((rng() * num_species as f64) as usize);
        fitness.push(if v > 0 { 1.5 } else if v < 0 { 0.5 } else { 1.0 });
    }

    // Species fitness multipliers (RPS-like advantage)
    let species_fitness = [1.2, 0.9, 1.1, 0.8, 1.0];

    for tick in 0..2000 {
        // Measure all 5 conservation candidates
        let pos = cells.iter().filter(|&&v| v == 1).count() as f64;
        let neg = cells.iter().filter(|&&v| v == -1).count() as f64;
        let zero = cells.iter().filter(|&&v| v == 0).count() as f64;
        let alive = grid_size as f64;

        // 1. gamma (ternary balance)
        let gamma = (pos - neg) / alive;

        // 2. Shannon entropy of ternary distribution
        let mut h = 0.0;
        for &count in &[pos, neg, zero] {
            if count > 0.0 {
                let p = count / alive;
                h -= p * p.log2();
            }
        }

        // 3. Total energy = sum of |values|
        let total_energy: f64 = cells.iter().map(|v| (v.abs() as f64)).sum();

        // 4. Species distribution entropy (information content)
        let mut species_counts = vec![0usize; num_species];
        for &s in &species { species_counts[s] += 1; }
        let mut info_content = 0.0;
        for &c in &species_counts {
            if c > 0 {
                let p = c as f64 / alive;
                info_content -= p * p.log2();
            }
        }

        // 5. Fitness-weighted entropy
        let weighted_sum: f64 = species.iter().zip(fitness.iter())
            .map(|(&s, &f)| f * species_fitness[s])
            .sum();
        let mut fitness_entropy = 0.0;
        for si in 0..num_species {
            let count = species.iter().filter(|&&s| s == si).count() as f64;
            if count > 0.0 && weighted_sum > 0.0 {
                let avg_fit: f64 = species.iter().zip(fitness.iter())
                    .filter(|(s, _)| **s == si)
                    .map(|(_, &f)| f * species_fitness[si])
                    .sum::<f64>();
                let p = avg_fit / weighted_sum;
                if p > 0.0 { fitness_entropy -= p * p.log2(); }
            }
        }

        // 6. Population variance (stability measure)
        let mean_count = alive / num_species as f64;
        let pop_variance: f64 = species_counts.iter()
            .map(|&c| { let d = c as f64 - mean_count; d * d })
            .sum::<f64>() / num_species as f64;

        // Print CSV
        println!("{},{:.4},{:.4},{:.4},{:.1},{:.4},{:.4},{:.1},{},{},{},{},{}",
            tick, gamma, h, gamma + h, total_energy, info_content, fitness_entropy, pop_variance,
            species_counts[0], species_counts[1], species_counts[2], species_counts[3], species_counts[4]);

        // Evolution step: selection + mutation
        // Selection: replace bottom 10% with copies of top 10%
        let mut indexed_fitness: Vec<(usize, f64)> = fitness.iter().cloned().enumerate().collect();
        indexed_fitness.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let elite = grid_size / 10;
        for i in 0..elite {
            let donor = indexed_fitness[i].0;
            let recipient = indexed_fitness[grid_size - 1 - i].0;
            cells[recipient] = cells[donor];
            species[recipient] = species[donor];
            fitness[recipient] = fitness[donor];
        }

        // Mutation: 2% chance per cell to flip
        for i in 0..grid_size {
            if rng() < 0.02 {
                cells[i] = match cells[i] { -1 => 0, 0 => 1, _ => -1 };
            }
            // Species mutation: 1% chance
            if rng() < 0.01 {
                species[i] = (rng() * num_species as f64) as usize;
            }
            // Update fitness
            fitness[i] = if cells[i] > 0 { 1.5 } else if cells[i] < 0 { 0.5 } else { 1.0 };
            fitness[i] *= species_fitness[species[i]];
        }
    }

    // Print summary
    eprintln!("\n=== CONSERVATION LAW HUNT SUMMARY ===");
    eprintln!("Ran 2000 ticks of ternary evolution on 100-cell grid with 5 species");
    eprintln!("Check CSV columns for drift in each conservation candidate");
    eprintln!("gamma+H: FAILED (known from previous experiment)");
    eprintln!("Look for columns with low variance — those are your conservation laws");
}

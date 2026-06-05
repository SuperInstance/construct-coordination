use std::collections::HashMap;

fn main() {
    let n = 100usize;
    let ticks = 2000usize;
    let num_species = 5usize;
    let mutation_rate = 0.02f64;
    let species_switch_rate = 0.01f64;
    let mi_window = 50usize;

    let mut rng_state: u64 = 42;
    let mut rng = || -> f64 {
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (rng_state >> 33) as f64 / (1u64 << 31) as f64
    };

    // Initialize grid
    let mut values: Vec<i8> = (0..n).map(|_| {
        let r = rng();
        if r < 0.333 { -1 } else if r < 0.667 { 0 } else { 1 }
    }).collect();
    let mut species: Vec<usize> = (0..n).map(|_| (rng() * num_species as f64) as usize).collect();

    // History buffer for MI calculation (ring buffer per agent)
    let mut history: Vec<Vec<i8>> = vec![Vec::new(); n];

    // CSV header
    println!("tick,gamma,H,I_total,omega,gamma_plus_H,alive,species_1,species_2,species_3,species_4,species_5");

    for tick in 0..ticks {
        // Compute gamma (ternary imbalance)
        let gamma: f64 = values.iter().map(|&v| v as f64).sum::<f64>() / n as f64;

        // Compute H (Shannon entropy over species×state joint distribution)
        let mut joint_counts: HashMap<(usize, i8), usize> = HashMap::new();
        for i in 0..n {
            *joint_counts.entry((species[i], values[i])).or_insert(0) += 1;
        }
        let mut h = 0.0f64;
        for &count in joint_counts.values() {
            if count > 0 {
                let p = count as f64 / n as f64;
                h -= p * p.ln();
            }
        }

        // Compute I_total (pairwise MI using sliding window)
        let mut i_total = 0.0f64;
        let mut pair_count = 0usize;

        // Sample pairs to keep computation tractable (every 10th pair)
        for i in (0..n).step_by(3) {
            for j in ((i+1)..n).step_by(5) {
                let len_i = history[i].len().min(mi_window);
                let len_j = history[j].len().min(mi_window);
                let len = len_i.min(len_j);
                if len < 10 { continue; }

                // Joint and marginal distributions
                let mut joint: [[usize; 3]; 3] = [[0; 3]; 3];
                let mut marginal_i = [0usize; 3];
                let mut marginal_j = [0usize; 3];

                let start_i = history[i].len().saturating_sub(len);
                let start_j = history[j].len().saturating_sub(len);

                for k in 0..len {
                    let vi = (history[i][start_i + k] + 1) as usize; // map {-1,0,1} -> {0,1,2}
                    let vj = (history[j][start_j + k] + 1) as usize;
                    joint[vi][vj] += 1;
                    marginal_i[vi] += 1;
                    marginal_j[vj] += 1;
                }

                let total = len as f64;
                let mut mi = 0.0f64;
                for a in 0..3 {
                    for b in 0..3 {
                        if joint[a][b] > 0 && marginal_i[a] > 0 && marginal_j[b] > 0 {
                            let p_ab = joint[a][b] as f64 / total;
                            let p_a = marginal_i[a] as f64 / total;
                            let p_b = marginal_j[b] as f64 / total;
                            mi += p_ab * (p_ab / (p_a * p_b)).ln();
                        }
                    }
                }
                i_total += mi;
                pair_count += 1;
            }
        }
        if pair_count > 0 { i_total /= pair_count as f64; } // Average MI per pair

        let omega = gamma.abs() + h + i_total;
        let gamma_plus_h = gamma + h;

        // Species counts
        let mut sc = vec![0usize; num_species];
        for &s in &species { sc[s] += 1; }
        let alive = values.iter().filter(|&&v| v != 0).count() as f64 / n as f64;

        println!("{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.4},{},{},{},{},{}",
            tick, gamma, h, i_total, omega, gamma_plus_h, alive,
            sc[0], sc[1], sc[2], sc[3], sc[4]);

        // Evolution step: majority-rule with neighbors
        let mut new_values = values.clone();
        let mut new_species = species.clone();
        for i in 0..n {
            // Moore neighborhood (wrapping)
            let mut neighbor_sum = 0i32;
            let mut count = 0i32;
            for di in -1i32..=1 {
                for dj in -1i32..=1 {
                    if di == 0 && dj == 0 { continue; }
                    let ni = ((i as i32 + di).rem_euclid(10)) as usize * 10 + 
                             ((i % 10) as i32 + dj).rem_euclid(10) as usize;
                    if ni < n {
                        neighbor_sum += values[ni] as i32;
                        count += 1;
                    }
                }
            }
            // Majority rule
            if neighbor_sum > 0 { new_values[i] = 1; }
            else if neighbor_sum < 0 { new_values[i] = -1; }
            else { /* keep current value */ }

            // Mutation
            if rng() < mutation_rate {
                let r = rng();
                new_values[i] = if r < 0.333 { -1 } else if r < 0.667 { 0 } else { 1 };
            }
            // Species switch
            if rng() < species_switch_rate {
                new_species[i] = (rng() * num_species as f64) as usize;
            }
        }
        values = new_values;
        species = new_species;

        // Record history
        for i in 0..n {
            history[i].push(values[i]);
            if history[i].len() > mi_window * 2 {
                history[i].drain(0..mi_window);
            }
        }
    }
}

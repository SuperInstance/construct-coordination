fn main() {
    let n = 10000usize;
    let ticks = 10000usize;
    let zero_transition_rate = 0.01f64;
    
    // Sweep tunneling rates (0 = no escape, 0.005 = slow, 0.01 = matched, 0.02 = fast)
    let tunneling_rates: Vec<(f64, &str)> = vec![
        (0.0, "none"),
        (0.003, "slow"),
        (0.006, "optimal_low"),
        (0.01, "matched"),
        (0.02, "fast"),
        (0.05, "very_fast"),
    ];

    println!("tunnel_rate,tick,signed_gamma,abs_gamma,H,frac_zero,abs_gamma_plus_H,survival_rate");

    for (tunnel_rate, tunnel_name) in &tunneling_rates {
        let mut rng_state: u64 = 42u64;
        let mut rng = || -> f64 {
            rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
            (rng_state >> 33) as f64 / (1u64 << 31) as f64
        };

        let mut values: Vec<i8> = vec![1; n];

        for tick in 0..ticks {
            if tick % 100 != 0 { 
                // Still evolve, just don't log every tick
                for i in 0..n {
                    if values[i] == 1 && rng() < zero_transition_rate {
                        values[i] = 0;
                    } else if values[i] == 0 && rng() < *tunnel_rate {
                        // Tunnel back out — randomly +1 or -1
                        values[i] = if rng() < 0.5 { 1 } else { -1 };
                    }
                }
                continue;
            }

            let signed_gamma: f64 = values.iter().map(|&v| v as f64).sum::<f64>() / n as f64;
            let abs_gamma: f64 = values.iter().map(|&v| (v as f64).abs()).sum::<f64>() / n as f64;
            let frac_zero: f64 = values.iter().filter(|&&v| v == 0).count() as f64 / n as f64;
            let survival_rate = 1.0 - frac_zero;

            // H over {-1, 0, +1}
            let mut counts = [0usize; 3];
            for &v in &values { counts[(v+1) as usize] += 1; }
            let mut h = 0.0f64;
            for &c in &counts {
                if c > 0 { let p = c as f64 / n as f64; h -= p * p.ln(); }
            }
            let abs_gamma_plus_h = abs_gamma + h;

            println!("{},{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}",
                tunnel_name, tick, signed_gamma, abs_gamma, h, frac_zero, abs_gamma_plus_h, survival_rate);

            // Evolve
            for i in 0..n {
                if values[i] == 1 && rng() < zero_transition_rate {
                    values[i] = 0;
                } else if values[i] == 0 && rng() < *tunnel_rate {
                    values[i] = if rng() < 0.5 { 1 } else { -1 };
                }
            }
        }
    }
}

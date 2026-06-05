fn main() {
    let n = 10000usize;
    let ticks = 5000usize;
    let zero_transition_rate = 0.01f64; // 1% chance per tick of +1→0
    
    let mut rng_state: u64 = 12345u64;
    let mut rng = || -> f64 {
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (rng_state >> 33) as f64 / (1u64 << 31) as f64
    };

    let mut values: Vec<i8> = vec![1; n]; // ALL +1, no mutation, no evolution
    let mut total_disappeared: i64 = 0; // Track charge that "vanished" from signed sum

    println!("tick,signed_gamma,abs_gamma,H,frac_zero,disappeared_charge,abs_gamma_plus_H");

    for tick in 0..ticks {
        let signed_gamma: f64 = values.iter().map(|&v| v as f64).sum::<f64>() / n as f64;
        let abs_gamma: f64 = values.iter().map(|&v| (v as f64).abs()).sum::<f64>() / n as f64;
        let frac_zero: f64 = values.iter().filter(|&&v| v == 0).count() as f64 / n as f64;
        
        // H over {+1, 0} distribution
        let n_pos = values.iter().filter(|&&v| v == 1).count();
        let n_zero = values.iter().filter(|&&v| v == 0).count();
        let mut h = 0.0f64;
        if n_pos > 0 { let p = n_pos as f64 / n as f64; h -= p * p.ln(); }
        if n_zero > 0 { let p = n_zero as f64 / n as f64; h -= p * p.ln(); }
        
        let abs_gamma_plus_h = abs_gamma + h;

        println!("{},{:.6},{:.6},{:.6},{:.6},{},{:.6}",
            tick, signed_gamma, abs_gamma, h, frac_zero, total_disappeared, abs_gamma_plus_h);

        // Only transition: +1 → 0 (charge doesn't disappear, it hides)
        for i in 0..n {
            if values[i] == 1 && rng() < zero_transition_rate {
                values[i] = 0;
                total_disappeared += 1;
            }
        }
    }
}

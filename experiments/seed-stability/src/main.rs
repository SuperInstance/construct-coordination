use ternary_seed::{Seed, SeedCombiner, SeedDecoder, SeedEncoder, SeedMutator, Trit};
use ternary_arena::{Agent, Arena, ArenaRules, Tournament};
use ternary_dice::{Prng};
use ternary_genome::{Chromosome, Crossover, Gene, Genome, MutationRate, Ternary};

use std::collections::HashMap;

/// Generate a deterministic seed with `n` entries using a PRNG.
fn make_base_seed(id: u64, n: usize, prng_seed: u32) -> Seed {
    let mut prng = Prng::new(prng_seed);
    let mut observations = Vec::with_capacity(n);
    for i in 0..n {
        let input_hash = (id as u64).wrapping_mul(1000).wrapping_add(i as u64);
        let trit = match prng.next_range(3) {
            0 => Trit::Neg,
            1 => Trit::Zero,
            _ => Trit::Pos,
        };
        observations.push((input_hash, trit));
    }
    let encoder = SeedEncoder::new();
    encoder.encode(id, &observations)
}

/// Hamming distance: count positions where two seeds differ on shared keys.
fn hamming_distance(a: &Seed, b: &Seed) -> (usize, usize) {
    let decoder = SeedDecoder::new();
    let map_a = decoder.decode(a);
    let map_b = decoder.decode(b);
    let all_keys: std::collections::HashSet<u64> = map_a.keys().chain(map_b.keys()).copied().collect();
    let total = all_keys.len();
    let diffs = all_keys.iter().filter(|&&k| {
        let va = map_a.get(&k).map(|t| t.value());
        let vb = map_b.get(&k).map(|t| t.value());
        va != vb
    }).count();
    (diffs, total)
}

/// Check conservation law compliance: gamma + entropy target check.
fn check_conservation(seed: &Seed) -> (f64, f64, bool) {
    let decoder = SeedDecoder::new();
    let map = decoder.decode(seed);
    if map.is_empty() {
        return (0.0, 0.0, true);
    }

    let total = map.len() as f64;
    let neg_count = map.values().filter(|&&t| t == Trit::Neg).count() as f64;
    let pos_count = map.values().filter(|&&t| t == Trit::Pos).count() as f64;
    let zero_count = map.values().filter(|&&t| t == Trit::Zero).count() as f64;

    // gamma = suppression ratio
    let gamma = neg_count / total;

    // Shannon entropy of the trit distribution
    let mut entropy = 0.0;
    for &count in &[neg_count, zero_count, pos_count] {
        if count > 0.0 {
            let p = count / total;
            entropy -= p * p.log2();
        }
    }

    // Conservation target: gamma + entropy ≈ 1.283 - 0.159 * ln(V)
    let v = total;
    let target = 1.283 - 0.159 * v.ln();
    let actual = gamma + entropy;
    let valid = (actual - target).abs() < 0.5; // relaxed tolerance for these small seeds

    (gamma, entropy, valid)
}

/// Count how many trits changed value in a mutation.
fn output_changes(original: &Seed, mutated: &Seed) -> usize {
    let decoder = SeedDecoder::new();
    let map_orig = decoder.decode(original);
    let map_mut = decoder.decode(mutated);
    let mut changes = 0;
    for (&k, &v_orig) in &map_orig {
        if let Some(&v_mut) = map_mut.get(&k) {
            if v_orig != v_mut {
                changes += 1;
            }
        }
    }
    changes
}

// ── Experiment 1: Mutation Stability ──────────────────────────────────

fn experiment_mutation_stability() -> String {
    let mut report = String::new();
    report.push_str("# Experiment 1: Seed Stability Under Mutation\n\n");
    report.push_str("| Seed | Rate | Avg Hamming | Avg Output Changes | Conservation % |\n");
    report.push_str("|------|------|-------------|-------------------|----------------|\n");

    let seed_sizes = 64; // entries per seed
    let num_seeds: u64 = 10;
    let mutations_per_rate = 100;
    let mutation_rates: &[f64] = &[0.01, 0.05, 0.10, 0.20, 0.50];

    let _summary_data: Vec<(f64, f64, f64, f64)> = Vec::new();

    for seed_idx in 0..num_seeds {
        let base_seed = make_base_seed(seed_idx, seed_sizes, (seed_idx as u32) * 7919 + 42);

        for &rate in mutation_rates {
            let mut total_hamming = 0usize;
            let mut total_changes = 0usize;
            let mut conserv_ok = 0usize;

            for trial in 0..mutations_per_rate {
                let mut mutated = base_seed.clone();
                let mutator = SeedMutator::new(rate);
                // Use deterministic counter per trial
                let base_counter = (seed_idx as u64) * 10000 + trial as u64 * 137;
                mutator.mutate_all(&mut mutated, base_counter);

                let (diffs, _total) = hamming_distance(&base_seed, &mutated);
                total_hamming += diffs;
                total_changes += output_changes(&base_seed, &mutated);

                let (_, _, valid) = check_conservation(&mutated);
                if valid { conserv_ok += 1; }
            }

            let avg_hamming = total_hamming as f64 / mutations_per_rate as f64;
            let avg_changes = total_changes as f64 / mutations_per_rate as f64;
            let conserv_pct = conserv_ok as f64 / mutations_per_rate as f64 * 100.0;

            report.push_str(&format!(
                "| S{} | {:.0}% | {:.1} | {:.1} | {:.0}% |\n",
                seed_idx, rate * 100.0, avg_hamming, avg_changes, conserv_pct
            ));
        }
    }

    // Aggregate summary
    report.push_str("\n## Aggregated Results by Mutation Rate\n\n");
    report.push_str("| Rate | Mean Hamming | Mean Output Changes | Conservation Compliance |\n");
    report.push_str("|------|-------------|--------------------|-----------------------|\n");

    // Recompute aggregated
    for &rate in mutation_rates {
        let mut total_h = 0.0f64;
        let mut total_c = 0.0f64;
        let mut total_cons = 0.0f64;
        let count = 10.0f64;

        for seed_idx in 0..num_seeds {
            let base_seed = make_base_seed(seed_idx, seed_sizes, (seed_idx as u32) * 7919 + 42);
            let mut th = 0usize;
            let mut tc = 0usize;
            let mut tcon = 0usize;

            for trial in 0..mutations_per_rate {
                let mut mutated = base_seed.clone();
                let mutator = SeedMutator::new(rate);
                let base_counter = (seed_idx as u64) * 10000 + trial as u64 * 137;
                mutator.mutate_all(&mut mutated, base_counter);
                let (d, _) = hamming_distance(&base_seed, &mutated);
                th += d;
                tc += output_changes(&base_seed, &mutated);
                let (_, _, v) = check_conservation(&mutated);
                if v { tcon += 1; }
            }
            total_h += th as f64 / mutations_per_rate as f64;
            total_c += tc as f64 / mutations_per_rate as f64;
            total_cons += tcon as f64 / mutations_per_rate as f64 * 100.0;
        }

        report.push_str(&format!(
            "| {:.0}% | {:.2} | {:.2} | {:.1}% |\n",
            rate * 100.0,
            total_h / count,
            total_c / count,
            total_cons / count
        ));
    }

    report
}

// ── Experiment 2: Cross-Breeding ─────────────────────────────────────

fn experiment_crossbreeding() -> String {
    let mut report = String::new();
    report.push_str("# Experiment 2: Seed Cross-Breeding\n\n");

    let seed_size = 64;
    let combiner = SeedCombiner::new();
    let decoder = SeedDecoder::new();

    // Create 10 base seeds
    let seeds: Vec<Seed> = (0..10u64)
        .map(|i| make_base_seed(i, seed_size, (i as u32) * 7919 + 42))
        .collect();

    report.push_str("## Parent Profiles\n\n");
    for (i, s) in seeds.iter().enumerate() {
        let (gamma, entropy, valid) = check_conservation(s);
        let map = decoder.decode(s);
        let neg = map.values().filter(|&&t| t == Trit::Neg).count();
        let zero = map.values().filter(|&&t| t == Trit::Zero).count();
        let pos = map.values().filter(|&&t| t == Trit::Pos).count();
        report.push_str(&format!(
            "- **Seed {}**: {} entries (Neg={}, Zero={}, Pos={}), γ={:.3}, H={:.3}, valid={}\n",
            i, map.len(), neg, zero, pos, gamma, entropy, valid
        ));
    }

    // Cross-breed pairs
    report.push_str("\n## Offspring Analysis\n\n");
    report.push_str("| Parent A | Parent B | Offspring Entries | Shared Keys | Overlap → Zero | Hamming(A) | Hamming(B) | Valid |\n");
    report.push_str("|----------|----------|-------------------|-------------|---------------|------------|------------|-------|\n");

    let pairs: Vec<(usize, usize)> = vec![
        (0, 1), (0, 2), (1, 3), (2, 4), (3, 5),
        (4, 6), (5, 7), (6, 8), (7, 9), (0, 9),
    ];

    for (ai, bi) in &pairs {
        let a = &seeds[*ai];
        let b = &seeds[*bi];
        let offspring = combiner.combine(a, b, 100 + *ai as u64 * 10 + *bi as u64);

        let map_a = decoder.decode(a);
        let map_b = decoder.decode(b);
        let map_off = decoder.decode(&offspring);

        let shared: std::collections::HashSet<u64> =
            map_a.keys().filter(|k| map_b.contains_key(k)).copied().collect();
        let overlap_to_zero = shared.iter().filter(|&&k| map_off.get(&k) == Some(&Trit::Zero)).count();

        let (ham_a, _) = hamming_distance(&offspring, a);
        let (ham_b, _) = hamming_distance(&offspring, b);
        let (_, _, valid) = check_conservation(&offspring);

        report.push_str(&format!(
            "| S{} | S{} | {} | {} | {} | {} | {} | {} |\n",
            ai, bi, map_off.len(), shared.len(), overlap_to_zero,
            ham_a, ham_b, valid
        ));
    }

    // Trait inheritance analysis
    report.push_str("\n## Trait Inheritance Patterns\n\n");
    for (ai, bi) in &pairs {
        let a = &seeds[*ai];
        let b = &seeds[*bi];
        let offspring = combiner.combine(a, b, 200 + *ai as u64 * 10 + *bi as u64);

        let map_a = decoder.decode(a);
        let map_b = decoder.decode(b);
        let map_off = decoder.decode(&offspring);

        let mut from_a = 0usize;
        let mut from_b = 0usize;
        let mut neutral = 0usize;

        for (&k, &v_off) in &map_off {
            let v_a = map_a.get(&k);
            let v_b = map_b.get(&k);
            match (v_a, v_b) {
                (Some(va), Some(vb)) => {
                    if v_off == *va && v_off != *vb { from_a += 1; }
                    else if v_off == *vb && v_off != *va { from_b += 1; }
                    else if v_off == Trit::Zero && va != vb { neutral += 1; }
                }
                (Some(_), None) => from_a += 1,
                (None, Some(_)) => from_b += 1,
                _ => {}
            }
        }

        report.push_str(&format!(
            "- **S{} × S{}**: {} from A, {} from B, {} neutralized (conflicts → Zero)\n",
            ai, bi, from_a, from_b, neutral
        ));
    }

    report
}

// ── Experiment 3: Seed Tournament ────────────────────────────────────

fn experiment_tournament() -> String {
    let mut report = String::new();
    report.push_str("# Experiment 3: Seed Tournament\n\n");

    let seed_size = 32;
    let decoder = SeedDecoder::new();
    let rules = ArenaRules::new(); // Rock-paper-scissors style

    // Create 16 agents from seeds
    let mut agents = Vec::new();
    let mut seed_profiles: HashMap<u64, String> = HashMap::new();

    for i in 0..16u64 {
        let seed = make_base_seed(i, seed_size, (i as u32) * 3571 + 13);
        let map = decoder.decode(&seed);

        // Strategy: use the seed entries as move sequence
        let strategy: Vec<ternary_arena::Trit> = (0..seed_size)
            .map(|j| {
                let k = i * 1000 + j as u64;
                match map.get(&k) {
                    Some(ternary_seed::Trit::Neg) => ternary_arena::Trit::Neg,
                    Some(ternary_seed::Trit::Zero) => ternary_arena::Trit::Zero,
                    Some(ternary_seed::Trit::Pos) => ternary_arena::Trit::Pos,
                    None => ternary_arena::Trit::Zero,
                }
            })
            .collect();

        // Profile
        let neg = strategy.iter().filter(|&&t| t == ternary_arena::Trit::Neg).count();
        let zero = strategy.iter().filter(|&&t| t == ternary_arena::Trit::Zero).count();
        let pos = strategy.iter().filter(|&&t| t == ternary_arena::Trit::Pos).count();
        seed_profiles.insert(i, format!("Neg={} Zero={} Pos={}", neg, zero, pos));

        let name = format!("seed-{}", i);
        agents.push(Agent::new(i, &name, strategy));
    }

    // Profile summary
    report.push_str("## Agent Profiles (strategy trit distribution)\n\n");
    for i in 0..16u64 {
        report.push_str(&format!("- **Agent {}**: {}\n", i, seed_profiles.get(&i).unwrap()));
    }

    // Run tournament
    let mut arena = Arena::with_rules("Stability Tournament", rules.clone());
    let tournament = Tournament::new("seed-stability-cup", agents.clone(), 16);
    let champion = arena.play_tournament(tournament);

    report.push_str(&format!("\n## Tournament Results\n\n"));
    report.push_str(&format!("**Champion**: Agent {}\n\n", champion.map(|id| id.to_string()).unwrap_or_else(|| "Draw".to_string())));

    // Leaderboard
    let board = arena.scoreboard.leaderboard();
    report.push_str("### Leaderboard\n\n");
    report.push_str("| Rank | Agent | Score | Wins | Losses | Draws | Win Rate |\n");
    report.push_str("|------|-------|-------|------|--------|-------|----------|\n");

    for (rank, &(id, score)) in board.iter().enumerate() {
        let wins = arena.scoreboard.wins(id);
        let losses = arena.scoreboard.losses(id);
        let draws = arena.scoreboard.draws(id);
        let win_rate = arena.spectator.win_rate(id);
        report.push_str(&format!(
            "| {} | Agent {} | {} | {} | {} | {} | {:.1}% |\n",
            rank + 1, id, score, wins, losses, draws, win_rate * 100.0
        ));
    }

    // Trait analysis of survivors
    report.push_str("\n### Trait Analysis of Top Performers\n\n");
    let top_4: Vec<u64> = board.iter().take(4).map(|&(id, _)| id).collect();
    for &id in &top_4 {
        let profile = seed_profiles.get(&id).unwrap();
        let agent = agents.iter().find(|a| a.id == id).unwrap();
        let pos_count = agent.strategy.iter().filter(|&&t| t == ternary_arena::Trit::Pos).count();
        let neg_count = agent.strategy.iter().filter(|&&t| t == ternary_arena::Trit::Neg).count();
        report.push_str(&format!(
            "- **Agent {}** (Score: {}): {} — Pos ratio: {:.2}, Neg ratio: {:.2}\n",
            id,
            arena.scoreboard.score(id),
            profile,
            pos_count as f64 / agent.strategy.len() as f64,
            neg_count as f64 / agent.strategy.len() as f64
        ));
    }

    // Multiple tournament runs with shuffled seeding
    report.push_str("\n## Multi-Run Stability (5 shuffled tournaments)\n\n");
    let mut champion_counts: HashMap<u64, u32> = HashMap::new();
    for run in 0..5 {
        // Shuffle agents deterministically
        let mut shuffled = agents.clone();
        let mut prng = Prng::new(run as u32 * 997 + 17);
        for i in (1..shuffled.len()).rev() {
            let j = prng.next_range((i + 1) as u32) as usize;
            shuffled.swap(i, j);
        }
        let mut arena2 = Arena::with_rules(&format!("Run {}", run), rules.clone());
        let t = Tournament::new(&format!("run-{}", run), shuffled, 16);
        let champ = arena2.play_tournament(t);
        if let Some(id) = champ {
            *champion_counts.entry(id).or_insert(0) += 1;
        }
    }

    report.push_str("| Agent | Championship Wins |\n");
    report.push_str("|-------|------------------|\n");
    let mut champs: Vec<_> = champion_counts.iter().collect();
    champs.sort_by(|a, b| b.1.cmp(a.1));
    for (&id, &count) in &champs {
        report.push_str(&format!("| Agent {} | {} |\n", id, count));
    }

    report
}

// ── Experiment 4: Genome-Based Evolution ─────────────────────────────

fn experiment_genome_evolution() -> String {
    let mut report = String::new();
    report.push_str("# Experiment 4: Genome Evolution Under Selection Pressure\n\n");

    let gene_count = 32;
    let population_size = 20;
    let generations = 50;

    // Create initial population
    let mut rng_state = 42u64;
    let mut population: Vec<Genome> = Vec::new();

    for i in 0..population_size {
        let genes: Vec<Gene> = (0..gene_count)
            .map(|j| {
                let allele = Ternary::random(&mut rng_state);
                Gene::new(&format!("g{}", j), allele, 1)
            })
            .collect();
        let chrom = Chromosome::new(&format!("c{}", i), genes);
        population.push(Genome::new(vec![chrom]));
    }

    report.push_str(&format!(
        "Initial population: {} genomes × {} genes = {} total ternary alleles\n\n",
        population_size, gene_count, population_size * gene_count
    ));

    // Track fitness over generations
    let mut fitness_history: Vec<(usize, f64, f64, f64)> = Vec::new(); // (gen, min, avg, max)
    let mut mutation_rate = MutationRate::new(50); // 5%

    for gen in 0..generations {
        let fitnesses: Vec<i32> = population.iter().map(|g| g.fitness()).collect();
        let min_f = *fitnesses.iter().min().unwrap();
        let max_f = *fitnesses.iter().max().unwrap();
        let avg_f = fitnesses.iter().sum::<i32>() as f64 / fitnesses.len() as f64;

        fitness_history.push((gen, min_f as f64, avg_f, max_f as f64));

        // Selection: keep top 50%
        let mut indexed: Vec<(usize, i32)> = fitnesses.iter().enumerate().map(|(i, &f)| (i, f)).collect();
        indexed.sort_by(|a, b| b.1.cmp(&a.1));
        let survivors: Vec<Genome> = indexed.iter()
            .take(population_size / 2)
            .map(|&(i, _)| population[i].clone())
            .collect();

        // Check improvement for mutation rate adaptation
        if gen > 0 {
            let prev_avg = fitness_history[gen - 1].2;
            mutation_rate.adapt(avg_f > prev_avg);
        }

        // Reproduce: crossover + mutate
        let mut new_pop = survivors.clone();
        let mut rng_for_cross = rng_state;
        while new_pop.len() < population_size {
            let pa = rng_for_cross.wrapping_mul(6364136223846793005).wrapping_add(1);
            rng_for_cross = pa;
            let pb = rng_for_cross.wrapping_mul(6364136223846793005).wrapping_add(1);
            rng_for_cross = pb;
            let ia = (pa as usize) % survivors.len();
            let ib = (pb as usize) % survivors.len();
            let (child, _) = Crossover::single_point(&survivors[ia], &survivors[ib], gene_count / 2);
            new_pop.push(child);
        }

        // Mutate
        for genome in &mut new_pop[population_size / 2..] {
            genome.mutate(mutation_rate.rate_per_thousand, &mut rng_state);
        }

        population = new_pop;
    }

    // Report fitness trajectory
    report.push_str("## Fitness Over Generations\n\n");
    report.push_str("| Gen | Min Fitness | Avg Fitness | Max Fitness | Mutation Rate |\n");
    report.push_str("|-----|-------------|-------------|-------------|---------------|\n");
    for &(gen, min, avg, max) in fitness_history.iter().step_by(5) {
        let mr = if gen < 45 { format!("{:.1}%", mutation_rate.fraction() * 100.0) } else { "—".to_string() };
        report.push_str(&format!("| {} | {:.1} | {:.2} | {:.1} | {} |\n", gen, min, avg, max, mr));
    }
    // Last generation
    if let &(gen, min, avg, max) = fitness_history.last().unwrap() {
        report.push_str(&format!("| {} | {:.1} | {:.2} | {:.1} | {:.1}% |\n", gen, min, avg, max, mutation_rate.fraction() * 100.0));
    }

    // Final allele distribution
    let final_alleles: Vec<Ternary> = population.iter().flat_map(|g| g.alleles()).collect();
    let neg_count = final_alleles.iter().filter(|&&a| a == Ternary::Neg).count();
    let zero_count = final_alleles.iter().filter(|&&a| a == Ternary::Zero).count();
    let pos_count = final_alleles.iter().filter(|&&a| a == Ternary::Pos).count();
    let total = final_alleles.len();

    report.push_str(&format!(
        "\n## Final Population Allele Distribution\n\n- Neg: {} ({:.1}%)\n- Zero: {} ({:.1}%)\n- Pos: {} ({:.1}%)\n",
        neg_count, neg_count as f64 / total as f64 * 100.0,
        zero_count, zero_count as f64 / total as f64 * 100.0,
        pos_count, pos_count as f64 / total as f64 * 100.0
    ));

    report.push_str(&format!(
        "\nFitness improvement: {:.1} → {:.2} (avg), {:.1} → {:.1} (max)\n",
        fitness_history[0].2, fitness_history.last().unwrap().2,
        fitness_history[0].3, fitness_history.last().unwrap().3
    ));

    report
}

// ── Main ─────────────────────────────────────────────────────────────

fn main() {
    println!("=== Seed Stability Under Mutation ===\n");
    let report1 = experiment_mutation_stability();
    println!("{}", report1);

    println!("\n=== Cross-Breeding ===\n");
    let report2 = experiment_crossbreeding();
    println!("{}", report2);

    println!("\n=== Tournament ===\n");
    let report3 = experiment_tournament();
    println!("{}", report3);

    println!("\n=== Genome Evolution ===\n");
    let report4 = experiment_genome_evolution();
    println!("{}", report4);

    // Write combined report
    let full_report = format!(
        "# Seed Stability Experiment Results\n\n\
        Generated from ternary-seed, ternary-genome, ternary-dice, ternary-arena crates.\n\
        Seed size: 64 entries (ternary trits). Population: 10 base seeds.\n\n\
        ---\n\n{}\n\n---\n\n{}\n\n---\n\n{}\n\n---\n\n{}",
        report1, report2, report3, report4
    );

    std::fs::write("RESULTS.md", &full_report).expect("Failed to write RESULTS.md");
    println!("\n✓ Full results written to RESULTS.md");
}

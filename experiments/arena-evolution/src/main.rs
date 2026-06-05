use ternary_arena::{Agent, Arena, Match as ArenaMatch, Trit};
use ternary_dice::{Dice, Prng, Trit as DTrit};

use std::collections::HashMap;

const POP_SIZE: usize = 16;
const STRATEGY_LEN: usize = 8;
const ROUNDS_PER_MATCH: usize = 24;
const NUM_GENERATIONS: usize = 10;
const TOP_SURVIVORS: usize = 4;
const MUTATION_RATE: f64 = 0.25;

#[derive(Clone, Debug)]
struct Competitor {
    id: u64,
    strategy: Vec<Trit>,
    generation_born: usize,
    label: String,
}

impl Competitor {
    fn to_agent(&self) -> Agent {
        Agent::new(self.id, &self.label, self.strategy.clone())
    }

    /// Describe strategy as a compact string like "++0-0--"
    fn strategy_str(&self) -> String {
        self.strategy
            .iter()
            .map(|t| match t {
                Trit::Neg => '-',
                Trit::Zero => '0',
                Trit::Pos => '+',
            })
            .collect()
    }

    /// Dominant bias: count each trit type
    fn bias(&self) -> (usize, usize, usize) {
        let neg = self.strategy.iter().filter(|&&t| t == Trit::Neg).count();
        let zero = self.strategy.iter().filter(|&&t| t == Trit::Zero).count();
        let pos = self.strategy.iter().filter(|&&t| t == Trit::Pos).count();
        (neg, zero, pos)
    }

    fn dominant(&self) -> &'static str {
        let (n, z, p) = self.bias();
        if p > n && p > z { "Aggressive(+)" }
        else if n > p && n > z { "Defensive(-)" }
        else if z > p && z > n { "Neutral(0)" }
        else if p == n && p > z { "Mixed(+/-)" }
        else if p == z && p > n { "Mixed(+/0)" }
        else if n == z && n > p { "Mixed(-/0)" }
        else { "Balanced" }
    }
}

struct ExperimentResult {
    generation: usize,
    agent_id: u64,
    strategy: String,
    dominant: String,
    wins: u32,
    losses: u32,
    draws: u32,
    fitness: f64,
    generation_born: usize,
}

fn random_strategy(prng: &mut Prng) -> Vec<Trit> {
    let trits = [Trit::Neg, Trit::Zero, Trit::Pos];
    (0..STRATEGY_LEN)
        .map(|_| {
            let idx = prng.next_range(3) as usize;
            trits[idx]
        })
        .collect()
}

fn mutate_strategy(strategy: &[Trit], prng: &mut Prng) -> Vec<Trit> {
    let trits = [Trit::Neg, Trit::Zero, Trit::Pos];
    strategy
        .iter()
        .map(|&t| {
            if (prng.next_range(1000) as f64 / 1000.0) < MUTATION_RATE {
                // Pick a different trit
                let alternatives: Vec<Trit> = trits.iter().filter(|&&a| a != t).copied().collect();
                alternatives[prng.next_range(alternatives.len() as u32) as usize]
            } else {
                t
            }
        })
        .collect()
}

/// Run a round-robin tournament with stochastic dice noise applied to moves.
fn run_tournament(competitors: &[Competitor], seed: u32) -> HashMap<u64, (u32, u32, u32, f64)> {
    let mut arena = Arena::new("evolution-arena");
    let mut results: HashMap<u64, (u32, u32, u32)> = HashMap::new();
    let mut total_points: HashMap<u64, i32> = HashMap::new();

    // Dice for stochastic perturbation
    let mut dice = Dice::new(seed);

    for i in 0..competitors.len() {
        for j in (i + 1)..competitors.len() {
            let a = competitors[i].clone();
            let b = competitors[j].clone();

            // Create a noisy match: with some probability, dice overrides the agent's move
            let mut noisy_a_strategy = a.strategy.clone();
            let mut noisy_b_strategy = b.strategy.clone();

            // Apply dice noise to some positions (10% chance per position)
            for pos in 0..STRATEGY_LEN {
                if dice.roll() == DTrit::Zero {
                    // Random perturbation event
                    let noise_a = dice.roll();
                    let noise_b = dice.roll();
                    noisy_a_strategy[pos % noisy_a_strategy.len()] = match noise_a {
                        DTrit::Neg => Trit::Neg, DTrit::Zero => Trit::Zero, DTrit::Pos => Trit::Pos,
                    };
                    noisy_b_strategy[pos % noisy_b_strategy.len()] = match noise_b {
                        DTrit::Neg => Trit::Neg, DTrit::Zero => Trit::Zero, DTrit::Pos => Trit::Pos,
                    };
                }
            }

            let agent_a = Agent::new(a.id, &a.label, noisy_a_strategy);
            let agent_b = Agent::new(b.id, &b.label, noisy_b_strategy);

            let mut m = ArenaMatch::new(agent_a, agent_b, ROUNDS_PER_MATCH);
            arena.run_match_mut(&mut m);

            let (a_pts, b_pts) = m.total_points();
            *total_points.entry(a.id).or_insert(0) += a_pts;
            *total_points.entry(b.id).or_insert(0) += b_pts;

            match m.winner() {
                Some(winner_id) => {
                    let loser_id = if winner_id == a.id { b.id } else { a.id };
                    let (w, _, _) = results.entry(winner_id).or_insert((0, 0, 0));
                    *w += 1;
                    let (_, l, _) = results.entry(loser_id).or_insert((0, 0, 0));
                    *l += 1;
                }
                None => {
                    let ( _, _, d1) = results.entry(a.id).or_insert((0, 0, 0));
                    *d1 += 1;
                    let (_, _, d2) = results.entry(b.id).or_insert((0, 0, 0));
                    *d2 += 1;
                }
            }
        }
    }

    // Calculate fitness: weighted score
    let mut out = HashMap::new();
    for comp in competitors {
        let (w, l, d) = results.get(&comp.id).copied().unwrap_or((0, 0, 0));
        let pts = total_points.get(&comp.id).copied().unwrap_or(0);
        let fitness = (w as f64 * 3.0) + (d as f64 * 1.0) + (pts as f64 * 0.01);
        out.insert(comp.id, (w, l, d, fitness));
    }
    out
}

/// Analyze strategy diversity using Shannon entropy
fn strategy_entropy(competitors: &[Competitor]) -> f64 {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for c in competitors {
        *counts.entry(c.strategy_str()).or_insert(0) += 1;
    }
    let total = competitors.len() as f64;
    if total == 0.0 { return 0.0; }
    counts.values().map(|&c| {
        let p = c as f64 / total;
        if p > 0.0 { -p * p.log2() } else { 0.0 }
    }).sum()
}

/// Count unique dominant types
fn count_dominant_types(competitors: &[Competitor]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for c in competitors {
        *counts.entry(c.dominant().to_string()).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let mut prng = Prng::new(42);
    let mut next_id: u64 = 1;

    // CSV output
    println!("generation,agent_id,strategy,dominant,wins,losses,draws,fitness,generation_born");

    let mut competitors: Vec<Competitor> = (0..POP_SIZE)
        .map(|i| {
            let id = next_id;
            next_id += 1;
            Competitor {
                id,
                strategy: random_strategy(&mut prng),
                generation_born: 0,
                label: format!("agent-{}", i),
            }
        })
        .collect();

    let mut all_results: Vec<ExperimentResult> = Vec::new();
    let mut diversity_log: Vec<(usize, f64, HashMap<String, usize>)> = Vec::new();

    for gen in 0..NUM_GENERATIONS {
        let seed = 1000 + gen as u32;
        let results = run_tournament(&competitors, seed);

        // Record results
        for comp in &competitors {
            let (w, l, d, fitness) = results.get(&comp.id).copied().unwrap_or((0, 0, 0, 0.0));
            println!(
                "{},{},\"{}\",\"{}\",{},{},{},{:.2},{}",
                gen, comp.id, comp.strategy_str(), comp.dominant(),
                w, l, d, fitness, comp.generation_born
            );
            all_results.push(ExperimentResult {
                generation: gen,
                agent_id: comp.id,
                strategy: comp.strategy_str(),
                dominant: comp.dominant().to_string(),
                wins: w,
                losses: l,
                draws: d,
                fitness,
                generation_born: comp.generation_born,
            });
        }

        // Diversity analysis
        let entropy = strategy_entropy(&competitors);
        let dominant_types = count_dominant_types(&competitors);
        diversity_log.push((gen, entropy, dominant_types.clone()));

        eprintln!(
            "Gen {:2}: entropy={:.3}, types={:?}",
            gen, entropy, dominant_types
        );

        // Selection: sort by fitness, keep top survivors
        let mut ranked: Vec<_> = competitors.iter().collect();
        ranked.sort_by(|a, b| {
            let fa = results.get(&a.id).map(|r| r.3).unwrap_or(0.0);
            let fb = results.get(&b.id).map(|r| r.3).unwrap_or(0.0);
            fb.partial_cmp(&fa).unwrap_or(std::cmp::Ordering::Equal)
        });

        let survivors: Vec<Competitor> = ranked[..TOP_SURVIVORS].iter().map(|c| (*c).clone()).collect();

        // Print top 4 for this gen
        eprintln!(
            "  Top 4: {}",
            survivors
                .iter()
                .map(|s| format!("{}({}:{:.1})", s.label, s.strategy_str(), results.get(&s.id).map(|r| r.3).unwrap_or(0.0)))
                .collect::<Vec<_>>()
                .join(", ")
        );

        // Create next generation: survivors + mutated offspring
        let mut next_gen: Vec<Competitor> = survivors;

        // Each survivor produces 3 mutated offspring (4 * 3 = 12, total 16)
        for survivor in &ranked[..TOP_SURVIVORS] {
            for _ in 0..3 {
                let id = next_id;
                next_id += 1;
                let mutated_strategy = mutate_strategy(&survivor.strategy, &mut prng);
                next_gen.push(Competitor {
                    id,
                    strategy: mutated_strategy,
                    generation_born: gen + 1,
                    label: format!("agent-{}", id),
                });
            }
        }

        competitors = next_gen;
    }

    // === ANALYSIS ===
    eprintln!("\n=== EVOLUTION ANALYSIS ===\n");

    // 1. Strategy convergence over time
    eprintln!("1. STRATEGY CONVERGENCE:");
    for (gen, entropy, types) in &diversity_log {
        eprintln!("   Gen {}: entropy={:.3}, types={:?}", gen, entropy, types);
    }

    // 2. Dominant strategy trends
    eprintln!("\n2. DOMINANT STRATEGY TRENDS:");
    let mut gen_winners: Vec<(usize, &ExperimentResult)> = Vec::new();
    for gen in 0..NUM_GENERATIONS {
        let gen_results: Vec<_> = all_results.iter().filter(|r| r.generation == gen).collect();
        if let Some(best) = gen_results.iter().max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap()) {
            gen_winners.push((gen, best));
            eprintln!(
                "   Gen {}: {} ({}) fitness={:.2} w/l/d={}/{}/{}",
                gen, best.strategy, best.dominant, best.fitness, best.wins, best.losses, best.draws
            );
        }
    }

    // 3. Rock-paper-scissors dynamics check
    eprintln!("\n3. ROCK-PAPER-SCISSORS DYNAMICS:");
    let final_results: Vec<_> = all_results.iter().filter(|r| r.generation == NUM_GENERATIONS - 1).collect();
    let final_types: HashMap<&str, Vec<&ExperimentResult>> = {
        let mut map: HashMap<&str, Vec<&ExperimentResult>> = HashMap::new();
        for r in &final_results {
            map.entry(&r.dominant as &str).or_default().push(r);
        }
        map
    };
    for (dtype, agents) in &final_types {
        let avg_fitness: f64 = agents.iter().map(|a| a.fitness).sum::<f64>() / agents.len() as f64;
        let avg_wins: f64 = agents.iter().map(|a| a.wins).sum::<u32>() as f64 / agents.len() as f64;
        eprintln!("   {}: {} agents, avg_fitness={:.2}, avg_wins={:.1}", dtype, agents.len(), avg_fitness, avg_wins);
    }
    if final_types.len() >= 3 {
        eprintln!("   → Multiple coexisting strategy types = rock-paper-scissors dynamics present!");
    } else if final_types.len() == 1 {
        eprintln!("   → CONVERGENCE: All strategies collapsed to one type!");
    }

    // 4. Novel strategies
    eprintln!("\n4. NOVEL STRATEGIES:");
    let initial_strategies: std::collections::HashSet<String> = all_results
        .iter()
        .filter(|r| r.generation_born == 0)
        .map(|r| r.strategy.clone())
        .collect();
    let mut novel_per_gen: HashMap<usize, Vec<String>> = HashMap::new();
    for r in &all_results {
        if !initial_strategies.contains(&r.strategy) && r.generation_born > 0 {
            novel_per_gen.entry(r.generation_born).or_default().push(r.strategy.clone());
        }
    }
    for (gen, strategies) in &novel_per_gen {
        let unique: std::collections::HashSet<&String> = strategies.iter().collect();
        eprintln!("   Born gen {}: {} novel unique strategies", gen, unique.len());
    }

    // 5. Fitness trajectory
    eprintln!("\n5. FITNESS TRAJECTORY (avg of top 4 per gen):");
    for gen in 0..NUM_GENERATIONS {
        let mut gen_results: Vec<_> = all_results.iter().filter(|r| r.generation == gen).collect();
        gen_results.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        let top4_avg: f64 = gen_results.iter().take(4).map(|r| r.fitness).sum::<f64>() / 4.0;
        let all_avg: f64 = gen_results.iter().map(|r| r.fitness).sum::<f64>() / gen_results.len().max(1) as f64;
        eprintln!("   Gen {}: top4_avg={:.2}, pop_avg={:.2}", gen, top4_avg, all_avg);
    }

    // 6. Role of randomness
    eprintln!("\n6. ROLE OF RANDOMNESS:");
    let final_entropy = diversity_log.last().map(|(_, e, _)| *e).unwrap_or(0.0);
    let initial_entropy = diversity_log.first().map(|(_, e, _)| *e).unwrap_or(0.0);
    if final_entropy >= initial_entropy * 0.8 {
        eprintln!("   Dice randomness MAINTAINS diversity (entropy stable/risen)");
    } else {
        eprintln!("   Dice randomness INSUFFICIENT to prevent convergence (entropy dropped)");
    }
    let avg_draws: f64 = final_results.iter().map(|r| r.draws as f64).sum::<f64>() / final_results.len().max(1) as f64;
    eprintln!("   Average draws per agent (final gen): {:.1} / {} matches", avg_draws, POP_SIZE - 1);

    // Write CSV to file
    let csv_path = "/home/phoenix/repos/construct-coordination/experiments/arena-evolution/results.csv";
    let mut csv = String::from("generation,agent_id,strategy,dominant,wins,losses,draws,fitness,generation_born\n");
    for r in &all_results {
        csv.push_str(&format!(
            "{},{},\"{}\",\"{}\",{},{},{},{:.2},{}\n",
            r.generation, r.agent_id, r.strategy, r.dominant,
            r.wins, r.losses, r.draws, r.fitness, r.generation_born
        ));
    }
    std::fs::write(csv_path, csv).expect("Failed to write CSV");
    eprintln!("\nCSV written to {}", csv_path);
}

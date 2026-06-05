//! Multi-Objective Seed Evolution Experiment
//!
//! Tests whether Pareto-based multi-objective fitness prevents the
//! convergence-to-homogeneity problem found in single-objective evolution.
//!
//! Objectives:
//!   A: Sum of trits (maximize Pos) — the objective that caused collapse before
//!   B: Shannon entropy of the genome (maximize diversity)
//!   C: Alternating trit pairs (reward complexity/pattern)

use std::collections::HashSet;

use ternary_genome::{Chromosome, Crossover, Gene, Genome, Ternary};
use ternary_fitness::{Entropy, TernaryStrategy};

const POP_SIZE: usize = 20;
const GENOME_LEN: usize = 32;
const GENERATIONS: usize = 50;
const MUTATION_RATE_PER_THOUSAND: u32 = 50; // 5%

/// An agent with a flat ternary genome and multi-objective scores.
#[derive(Clone)]
struct Agent {
    genes: Vec<i8>,
    fitness_a: f64,
    fitness_b: f64,
    fitness_c: f64,
    pareto_rank: usize,
}

impl Agent {
    fn random(seed: &mut u64) -> Self {
        let mut genes = Vec::with_capacity(GENOME_LEN);
        for _ in 0..GENOME_LEN {
            genes.push(Ternary::random(seed).to_i8());
        }
        let mut agent = Self {
            genes,
            fitness_a: 0.0,
            fitness_b: 0.0,
            fitness_c: 0.0,
            pareto_rank: 0,
        };
        agent.compute_objectives();
        agent
    }

    fn compute_objectives(&mut self) {
        // Objective A: sum of trits (maximize Pos)
        self.fitness_a = self.genes.iter().map(|&g| g as f64).sum();

        // Objective B: Shannon entropy of the genome
        let strategy = TernaryStrategy::new(self.genes.clone());
        self.fitness_b = Entropy::strategy_entropy(&strategy);

        // Objective C: number of alternating trit pairs
        // Alternating = adjacent trits differ AND neither is zero
        let mut alternating = 0;
        for w in self.genes.windows(2) {
            if w[0] != 0 && w[1] != 0 && w[0] != w[1] {
                alternating += 1;
            }
        }
        self.fitness_c = alternating as f64;
    }

    fn to_genome(&self) -> Genome {
        let genes: Vec<Gene> = self
            .genes
            .iter()
            .enumerate()
            .map(|(i, &g)| {
                let allele = Ternary::from_i8(g).unwrap_or(Ternary::Zero);
                Gene::new(&format!("g{}", i), allele, 1)
            })
            .collect();
        Genome::new(vec![Chromosome::new("main", genes)])
    }

    fn from_genome(genome: &Genome) -> Self {
        let genes: Vec<i8> = genome.alleles().iter().map(|t| t.to_i8()).collect();
        let mut agent = Self {
            genes,
            fitness_a: 0.0,
            fitness_b: 0.0,
            fitness_c: 0.0,
            pareto_rank: 0,
        };
        agent.compute_objectives();
        agent
    }
}

/// Check if agent `a` dominates agent `b`
fn dominates(a: &Agent, b: &Agent) -> bool {
    let mut at_least_one_better = false;
    for (va, vb) in [
        (a.fitness_a, b.fitness_a),
        (a.fitness_b, b.fitness_b),
        (a.fitness_c, b.fitness_c),
    ] {
        if va < vb - 1e-12 {
            return false;
        }
        if va > vb + 1e-12 {
            at_least_one_better = true;
        }
    }
    at_least_one_better
}

/// Compute Pareto ranks using non-dominated sorting. Returns front sizes.
fn non_dominated_sort(agents: &mut [Agent]) -> Vec<usize> {
    let n = agents.len();
    let mut domination_count = vec![0usize; n];
    let mut dominated_set: Vec<Vec<usize>> = vec![Vec::new(); n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if dominates(&agents[i], &agents[j]) {
                dominated_set[i].push(j);
            } else if dominates(&agents[j], &agents[i]) {
                domination_count[i] += 1;
            }
        }
    }

    let mut front_sizes = Vec::new();
    let mut current_front: Vec<usize> = (0..n)
        .filter(|&i| domination_count[i] == 0)
        .collect();

    let mut rank = 0;
    while !current_front.is_empty() {
        front_sizes.push(current_front.len());
        for &i in &current_front {
            agents[i].pareto_rank = rank;
        }

        let mut next_front = Vec::new();
        for &i in &current_front {
            for &j in &dominated_set[i] {
                domination_count[j] -= 1;
                if domination_count[j] == 0 {
                    next_front.push(j);
                }
            }
        }

        rank += 1;
        current_front = next_front;
    }

    front_sizes
}

fn count_unique(agents: &[Agent]) -> usize {
    let mut seen = HashSet::new();
    for a in agents {
        seen.insert(a.genes.clone());
    }
    seen.len()
}

fn xorshift(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    *seed
}

/// Tournament selection based on Pareto rank with crowding tiebreak.
fn tournament_select(agents: &[Agent], seed: &mut u64) -> usize {
    let a = (xorshift(seed) as usize) % agents.len();
    let b = (xorshift(seed) as usize) % agents.len();

    if agents[a].pareto_rank < agents[b].pareto_rank {
        a
    } else if agents[b].pareto_rank < agents[a].pareto_rank {
        b
    } else {
        let sa = agents[a].fitness_a + agents[a].fitness_b + agents[a].fitness_c;
        let sb = agents[b].fitness_a + agents[b].fitness_b + agents[b].fitness_c;
        if sa >= sb { a } else { b }
    }
}

/// Mutate agent genes in place.
fn mutate_agent(agent: &mut Agent, seed: &mut u64) {
    for gene in &mut agent.genes {
        if (xorshift(seed) % 1000) < MUTATION_RATE_PER_THOUSAND as u64 {
            *gene = Ternary::random(seed).to_i8();
        }
    }
    agent.compute_objectives();
}

fn main() {
    let mut seed = 42u64;
    let mut agents: Vec<Agent> = (0..POP_SIZE).map(|_| Agent::random(&mut seed)).collect();

    println!("generation,avg_fitness_a,avg_fitness_b,avg_fitness_c,num_pareto_front,diversity");

    for gen in 0..=GENERATIONS {
        let front_sizes = non_dominated_sort(&mut agents);
        let pareto_front_size = front_sizes.first().copied().unwrap_or(0);
        let unique = count_unique(&agents);

        let avg_a: f64 = agents.iter().map(|a| a.fitness_a).sum::<f64>() / POP_SIZE as f64;
        let avg_b: f64 = agents.iter().map(|a| a.fitness_b).sum::<f64>() / POP_SIZE as f64;
        let avg_c: f64 = agents.iter().map(|a| a.fitness_c).sum::<f64>() / POP_SIZE as f64;

        println!(
            "{},{:.3},{:.3},{:.3},{},{}",
            gen, avg_a, avg_b, avg_c, pareto_front_size, unique
        );

        if gen == GENERATIONS {
            break;
        }

        // Build next generation
        let mut next_gen: Vec<Agent> = Vec::with_capacity(POP_SIZE);

        // Elitism: keep Pareto front (rank 0)
        for a in &agents {
            if a.pareto_rank == 0 {
                next_gen.push(a.clone());
            }
        }

        // Fill rest with offspring via crossover + mutation
        while next_gen.len() < POP_SIZE {
            let pa = tournament_select(&agents, &mut seed);
            let pb = tournament_select(&agents, &mut seed);

            let ga = agents[pa].to_genome();
            let gb = agents[pb].to_genome();

            let point = (xorshift(&mut seed) as usize) % GENOME_LEN;
            let (child, _) = Crossover::single_point(&ga, &gb, point);

            let mut child_agent = Agent::from_genome(&child);
            mutate_agent(&mut child_agent, &mut seed);
            next_gen.push(child_agent);
        }

        next_gen.truncate(POP_SIZE);
        agents = next_gen;
    }

    // Final analysis
    eprintln!("\n=== FINAL ANALYSIS ===");
    let front_sizes = non_dominated_sort(&mut agents);
    let unique = count_unique(&agents);

    eprintln!("Unique genomes at gen {}: {}", GENERATIONS, unique);
    eprintln!("Pareto front size: {}", front_sizes.first().unwrap_or(&0));
    eprintln!("Number of Pareto ranks: {}", front_sizes.len());
    eprintln!("Front sizes: {:?}", front_sizes);

    eprintln!("\nPareto front members:");
    for a in &agents {
        if a.pareto_rank == 0 {
            let pos_count = a.genes.iter().filter(|&&g| g == 1).count();
            let neg_count = a.genes.iter().filter(|&&g| g == -1).count();
            let zero_count = a.genes.iter().filter(|&&g| g == 0).count();
            eprintln!(
                "  A={:.1} B={:.3} C={:.0} | Pos={} Zero={} Neg={}",
                a.fitness_a, a.fitness_b, a.fitness_c, pos_count, zero_count, neg_count
            );
        }
    }

    let (min_a, max_a) = agents.iter().fold((f64::MAX, f64::MIN), |(mn, mx), a| (mn.min(a.fitness_a), mx.max(a.fitness_a)));
    let (min_b, max_b) = agents.iter().fold((f64::MAX, f64::MIN), |(mn, mx), a| (mn.min(a.fitness_b), mx.max(a.fitness_b)));
    let (min_c, max_c) = agents.iter().fold((f64::MAX, f64::MIN), |(mn, mx), a| (mn.min(a.fitness_c), mx.max(a.fitness_c)));
    eprintln!("\nObjective ranges across all agents:");
    eprintln!("  A (sum): [{:.1}, {:.1}]", min_a, max_a);
    eprintln!("  B (entropy): [{:.3}, {:.3}]", min_b, max_b);
    eprintln!("  C (alternating): [{:.0}, {:.0}]", min_c, max_c);
}

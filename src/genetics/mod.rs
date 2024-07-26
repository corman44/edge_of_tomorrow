pub mod fitness;

use fitness::PopulationSum;
use genetic_algorithm::strategy::evolve::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

pub fn run_simulation() {
    let mut rng = SmallRng::from_entropy();
    let genotype: ContinuousGenotype = ContinuousGenotype::builder()
        .with_genes_size(16)
        .with_allele_range(-1.0..1.0)
        .build()
        .unwrap();

    println!("{}", genotype);

    let now = std::time::Instant::now();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(10)
        .with_mutate(MutateOnce::new(0.2))
        .with_fitness(PopulationSum(1e-5))
        .with_crossover(CrossoverUniform::new(true))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionNoop::new())
        .call(&mut rng)
        .unwrap();

    let duration = now.elapsed();

    println!("{}", evolve);
    println!("duration: {:?}", duration);
} 
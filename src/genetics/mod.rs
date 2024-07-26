pub mod fitness;

use std::ops::Range;

use fitness::ReproductiveScore;
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
    const POPULATION_SIZE: usize = 100;
    let mut reproduction_score: Range<i16> = 0..POPULATION_SIZE as i16;

    let evolve: Evolve<ContinuousGenotype, MutateOnce, ReproductiveScore, CrossoverUniform, CompeteTournament, ExtensionNoop> = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(10)
        .with_mutate(MutateOnce::new(0.2))
        .with_fitness(ReproductiveScore(reproduction_score))
        .with_crossover(CrossoverUniform::new(true))
        .with_compete(CompeteTournament::new(4))
        .with_extension(ExtensionNoop::new())
        .call(&mut rng)
        .unwrap();


    let duration = now.elapsed();

    println!("{}", evolve);
    println!("duration: {:?}", duration);
} 
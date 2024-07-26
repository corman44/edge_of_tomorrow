use std::ops::Range;

use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness::{Fitness, FitnessValue};
use genetic_algorithm::genotype::{ContinuousGenotype, Genotype};
#[derive(Clone, Debug)]
pub struct ReproductiveScore(pub Range<i16>);
impl Fitness for ReproductiveScore {
    type Genotype = ContinuousGenotype;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .map(|v| v / self.0)
                .sum::<<Self::Genotype as Genotype>::Allele>() as FitnessValue,
        )
    }
}


use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness::{Fitness, FitnessValue};
use genetic_algorithm::genotype::{ContinuousGenotype, ContinuousGenotypeAllele, Genotype};

pub type PopulationSumGenotypePrecision = ContinuousGenotypeAllele;
/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct PopulationSum(pub PopulationSumGenotypePrecision);
impl Fitness for PopulationSum {
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

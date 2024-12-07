use crate::genetics::Genome;
use rand::Rng;
use crate::params::MUTATION_RATE;

pub fn mutate(individual: &mut Genome) {
    let mut rng = rand::thread_rng();
    for i in 0..individual.len() {
        if rng.gen_bool(MUTATION_RATE) {
            let temp = !individual[i];
            individual.set(i, temp);
        }
    }
}

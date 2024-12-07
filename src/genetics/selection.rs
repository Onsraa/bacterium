use crate::genetics::Genome;
use rand::Rng;

pub fn roulette_wheel_selection(pop: &[(Genome, f64)], nb: usize) -> Vec<Genome> {
    let mut rng = rand::thread_rng();
    let total_fitness: f64 = pop.iter().map(|(_, f)| f).sum();
    let mut selected = Vec::new();
    for _ in 0..nb {
        let pick = rng.gen_range(0.0..total_fitness);
        let mut accum = 0.0;
        for (ind, fit) in pop {
            accum += fit;
            if accum >= pick {
                selected.push(*ind);
                break;
            }
        }
    }
    selected
}

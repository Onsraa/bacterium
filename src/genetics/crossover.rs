//! Crossover à un point aléatoire

use crate::genetics::Genome;
use rand::Rng;

pub fn random_crossover(parent_a: &Genome, parent_b: &Genome) -> (Genome, Genome) {
    let mut rng = rand::thread_rng();
    let point = rng.gen_range(1..parent_a.len());
    let mut child_a = *parent_a;
    let mut child_b = *parent_b;
    for i in point..child_a.len() {
        let temp = child_a[i];
        child_a.set(i, child_b[i]);
        child_b.set(i, temp);
    }
    (child_a, child_b)
}

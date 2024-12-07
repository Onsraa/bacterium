//! Calcul de la fitness

use crate::genetics::{Genome, interpret_behavior_from_color};

pub fn calculate_fitness(genome: &Genome) -> f64 {
    let (agg, soc) = interpret_behavior_from_color(genome);
    (agg + soc) as f64
}

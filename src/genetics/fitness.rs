//! Calcul de la fitness

use crate::genetics::{get_feeding_mode, Genome};
use crate::genetics::genome::{get_aggressivity, get_energy};

pub fn calculate_fitness(genome: &Genome) -> f64 {
    let bonus_carnivore: f64 = if get_feeding_mode(genome) == 1 {
        10.0
    }else { 0.0 };
    get_aggressivity(genome) as f64 + bonus_carnivore
}

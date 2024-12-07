use crate::genetics::Genome;
use bevy::prelude::*;
use crate::components::bacterium::FeedingMode::Vegetarian;

#[derive(Default, Debug)]
enum FeedingMode {
    #[default]
    Vegetarian,
    Carnivore,
    Omnivore,
}

#[derive(Component, Debug)]
pub struct Bacterium {
    pub genome: Genome,
    pub speed: f32,
    pub rotation_speed: f32,
    pub feeding_mode: FeedingMode,
}

impl Default for Bacterium {
    fn default() -> Self {
        Bacterium {
            genome: Genome::ZERO,
            speed: 0.0,
            rotation_speed: 0.0,
            feeding_mode: Vegetarian,
        }
    }
}

mod params;
mod genetics;
mod components;
mod systems;
mod plugins;
mod environment;
mod resources;
mod ui;

use bevy::prelude::*;
use crate::components::Bacterium;
// use crate::plugins::{SimulationPlugin, GeneticPlugin, UiPlugin};
// use crate::resources::GlobalSettings;

fn main() {
    App::new()
        //.add_plugins(DefaultPlugins)
        .add_plugins(SetupPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    println!("Setup start.");
}

fn check_bacteria(query: Query<(Entity, &Bacterium)>) {

    for (e, bacterium) in query.iter() {
        println!("Bacterium id : {}", e);
        println!("{:?}", bacterium);
    }
}

fn initialize_bacteria(mut commands: Commands) {
    let mut population = genetics::create_population(params::POPULATION_SIZE);
    for genome in population {
        commands.spawn(Bacterium {genome, ..default()});
    }
}

struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Startup, (initialize_bacteria, check_bacteria).chain());
    }
}
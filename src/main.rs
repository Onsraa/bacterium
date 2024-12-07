mod params;
mod genetics;
mod components;
mod systems;
mod plugins;
mod environment;
mod resources;
mod ui;

use bevy::{
    prelude::*,
    window::PresentMode,
    //diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
use rand::Rng;
use log::info;
use crate::components::{Bacterium, Food};
use crate::environment::{initialize_bacteria, spawn_food, spawn_population, PopulationResource};
use crate::genetics::{calculate_fitness, mutate, random_crossover, roulette_wheel_selection};
use crate::params::{POPULATION_SIZE, SIMULATION_DURATION};
// use crate::plugins::{SimulationPlugin, GeneticPlugin, UiPlugin};
// use crate::resources::GlobalSettings;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bacterium".into(),
                resolution: (params::WIDTH, params::HEIGHT).into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(SetupPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    info!("Start setup.");
    commands.spawn(Camera2d);
}

#[derive(Resource)]
struct GenerationTimer(Timer);
#[derive(Resource)]
struct GenerationCount(u32);

fn update_generation(
    mut commands: Commands,
    time: Res<Time>,
    mut generation_timer: ResMut<GenerationTimer>,
    mut generation_count: ResMut<GenerationCount>,
    mut population_resource: ResMut<PopulationResource>,
    bacterium : Query<(Entity, &Bacterium)>,
    foods : Query<Entity, With<Food>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    generation_timer.0.tick(time.delta());
    if generation_timer.0.finished() {
        println!("Génération {} terminée, évolution en cours..", generation_count.0);
        let mut pop_with_fitness = Vec::new();
        for (e, b) in bacterium.iter() {
            let fit = calculate_fitness(&b.genome);
            pop_with_fitness.push((b.genome, fit, e));
        }

        // On trie par fitness pour information (non obligatoire)
        pop_with_fitness.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());

        // Sélection de l’élite + roulette wheel (exemple simplifié)
        let elite_count = (POPULATION_SIZE as f64 * 0.1) as usize;
        let mut new_population = Vec::new();

        // On garde l’élite telle quelle (juste leurs génomes)
        for i in 0..elite_count {
            new_population.push(pop_with_fitness[i].0);
        }

        // On sélectionne le reste par roulette
        let parents = roulette_wheel_selection(&pop_with_fitness.iter().map(|(g,f,_)| (*g, *f)).collect::<Vec<_>>(), POPULATION_SIZE - elite_count);

        // On applique crossover et mutation
        let mut rng = rand::thread_rng();
        let mut parent_pool = parents;
        if parent_pool.len() % 2 != 0 {
            // Si impair, on ajoute un individu random
            parent_pool.push(pop_with_fitness[rng.gen_range(0..pop_with_fitness.len())].0);
        }
        for pair in parent_pool.chunks(2) {
            let p1 = pair[0];
            let p2 = pair[1];
            let (mut c1, mut c2) = random_crossover(&p1, &p2);

            mutate(&mut c1);
            mutate(&mut c2);

            if new_population.len() < POPULATION_SIZE {
                new_population.push(c1);
            }
            if new_population.len() < POPULATION_SIZE {
                new_population.push(c2);
            }
        }

        population_resource.population = Some(new_population);

        // On supprime l'ancienne population
        for (_,_,e) in pop_with_fitness {
            commands.entity(e).despawn();
        }

        for e in foods.iter() {
            commands.entity(e).despawn();
        }

        if let Some(_) = population_resource.population {
            spawn_population(&mut commands, &mut meshes, &mut materials, population_resource.population.as_ref().unwrap());
            spawn_food(&mut commands, &mut meshes, &mut materials);
        }

        generation_count.0 += 1;
        println!("Nouvelle génération : {}", generation_count.0);
    }
}

struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GenerationTimer(Timer::from_seconds(SIMULATION_DURATION, TimerMode::Repeating)));
        app.insert_resource(GenerationCount(0));
        app.insert_resource(PopulationResource::default());
        app.add_systems(Startup, setup);
        app.add_systems(Startup, initialize_bacteria);
        app.add_systems(Update, update_generation);
    }
}
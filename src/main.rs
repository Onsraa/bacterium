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
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};

use rand::Rng;

use crate::components::Bacterium;
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
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(SetupPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    println!("Setup start.");
}

// fn check_bacteria(query: Query<(Entity, &Bacterium)>) {
//
//     for (e, bacterium) in query.iter() {
//         println!("Bacterium id : {}", e);
//         println!("{:?}", bacterium);
//     }
// }

fn initialize_bacteria(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut population = genetics::create_population(params::POPULATION_SIZE);
    let mut rng = rand::thread_rng();

    for genome in population {
        let color = genetics::determine_color(&genome);


        let shape = meshes.add(Circle::new(5.0));
        let color = Color::from(color);

        let x = rng.gen_range(- params::WIDTH / 2.0..params::WIDTH / 2.0);
        let y = rng.gen_range(- params::HEIGHT / 2.0..params::HEIGHT / 2.0);

        commands.spawn((
            Bacterium {genome, ..default()},
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x, y,0.0),
        ));
    }
}

struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Startup, initialize_bacteria);
    }
}
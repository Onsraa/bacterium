use bevy::prelude::*;
use bevy::sprite::ColorMaterial;
use crate::components::{Bacterium, Food};
use crate::params::{WIDTH, HEIGHT, NUMBER_SPAWNED_FOOD, POPULATION_SIZE};
use rand::Rng;
use crate::{genetics, params};
use crate::genetics::{create_population, Genome};

pub fn initialize_bacteria(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>
) {
    let population = create_population(POPULATION_SIZE);
    spawn_population(commands, meshes, materials, &population);
}

pub fn spawn_population(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, population: &[Genome]) {

    let mut rng = rand::thread_rng();

    for genome in population.iter() {
        let color = genetics::determine_color(&genome);


        let shape = meshes.add(Circle::new(5.0));
        let color = Color::from(color);

        let x = rng.gen_range(- params::WIDTH / 2.0..params::WIDTH / 2.0);
        let y = rng.gen_range(- params::HEIGHT / 2.0..params::HEIGHT / 2.0);

        commands.spawn((
            Bacterium {genome: *genome, ..default()},
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x, y,0.0),
        ));
    }
}

pub fn spawn_food(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let mut rng = rand::thread_rng();

    for _ in 0..NUMBER_SPAWNED_FOOD {
        let mesh = meshes.add(Rectangle::new(2.5, 2.5));
        let x = rng.gen_range(-WIDTH/2.0..WIDTH/2.0);
        let y = rng.gen_range(-HEIGHT/2.0..HEIGHT/2.0);

        commands.spawn((
            Food::default(),
            Mesh2d(mesh),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz (x, y, 0.0)
        ));
    }
}

use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Food {
    value: f32
}

impl Default for Food {
    fn default() -> Self {
        Self {
            value: 10.0
        }
    }
}
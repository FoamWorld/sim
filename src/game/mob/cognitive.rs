use bevy::prelude::*;

#[derive(Component)]
pub struct Sanity {
    pub value: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Spirit {
    pub resistance: f32,
    pub sharpness: f32,
}

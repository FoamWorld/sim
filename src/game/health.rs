use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub f32, pub f32);

impl Health {
    pub fn fragile() -> Self {
        Self(0.0, 0.0)
    }
}

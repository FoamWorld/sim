use bevy::prelude::*;

#[derive(Event)]
pub struct HealthClearedEvent(pub Entity);

#[derive(Component)]
pub struct Health(pub f32, pub f32);

impl Health {
    pub fn fragile() -> Self {
        Self(1e-3, 1e-3)
    }
    pub fn modify(health: f32) {}
    pub fn shift(&mut self, shift: f32) {
        self.0 += shift;
        if self.0 < 1e-7 {}
    }
}

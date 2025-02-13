use bevy::prelude::*;

#[derive(Event)]
pub struct HealthClearedEvent(pub Entity);

pub fn read_health_cleared(
    mut commands: Commands,
    mut reader: EventReader<HealthClearedEvent>,
) {
    for health_cleared in reader.read() {
        let entity = health_cleared.0;
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
pub struct Health(pub f32, pub f32);

impl Health {
    pub fn fragile() -> Self {
        Self(1e-3, 1e-3)
    }
    pub fn modify(&mut self, health: f32) {
        self.0 = health;
    }
    pub fn shift(&mut self, shift: f32) -> f32 {
        self.0 += shift;
        if self.0 > self.1 {
            self.0 = self.1;
        }
        self.0
    }
}

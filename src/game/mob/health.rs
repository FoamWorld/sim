use bevy::prelude::*;

#[derive(Event)]
pub struct HealthClearedEvent(pub Entity);

pub fn read_health_cleared(mut commands: Commands, mut reader: EventReader<HealthClearedEvent>) {
    for health_cleared in reader.read() {
        let entity = health_cleared.0;
        if let Some(mut ec) = commands.get_entity(entity) {
            ec.despawn();
        }
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
#[type_path = "sim"]
pub struct Health {
    pub value: f32,
    pub max: f32,
}

impl Health {
    pub fn new(value: f32, max: f32) -> Self {
        Self { value, max }
    }

    /// Spawn with full health.
    pub fn fresh(val: f32) -> Self {
        Health::new(val, val)
    }

    /// For fragile objects that die with a single touch.
    pub fn fragile() -> Self {
        Health::fresh(1e-3)
    }

    pub fn is_alive(&self) -> bool {
        self.value > 1e-7
    }

    /// Updates health value.
    pub fn shift(&mut self, shift: f32) -> f32 {
        self.value += shift;
        if self.value > self.max {
            self.value = self.max;
        }
        self.value
    }
}

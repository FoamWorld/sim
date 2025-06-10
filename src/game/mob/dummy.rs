use bevy::prelude::*;

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::sensor"]
pub struct DamageSensor;

// pub struct AutoHealthRecovery;

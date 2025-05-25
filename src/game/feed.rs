use super::ecs::*;
use crate::assets::RpgTextures;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn feed_to_concrete(eidos: Entity, world: &World, mut ec: EntityCommands) {
    let entity_ref = world.entity(eidos);

    if let Some(icon) = entity_ref.get::<IconImage>() {
        let rpg_folder = world.resource::<RpgTextures>();
        icon.inserts_sprite(&mut ec, rpg_folder);
    }

    if let Some(physics) = entity_ref.get::<PhysicsConfig>() {
        ec.insert((
            RigidBody::Dynamic,
            Collider::ball(1.0),
            ColliderMassProperties::Mass(physics.mass),
        ));
    }
}

pub fn feed_to_attach(eidos: Entity, world: &World, mut ec: EntityCommands) {
    let entity_ref = world.entity(eidos);

    if let Some(icon) = entity_ref.get::<IconImage>() {
        let rpg_folder = world.resource::<RpgTextures>();
        icon.inserts_sprite(&mut ec, rpg_folder);
    }

    if let Some(holds) = entity_ref.get::<HoldsConfig>() {
        ec.insert((
            bevy::sprite::Anchor::Custom(holds.get_offset()),
            crate::physics::camera::RotateWithMouse::new(holds.get_rotate_range()),
        ));
    }
}

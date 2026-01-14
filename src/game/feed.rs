use super::ecs::*;
use crate::assets::MyTextures;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn feed_to_concrete(eidos: Entity, world: &World, mut ec: EntityCommands) {
    let entity_ref = world.entity(eidos);

    if let Some(icon) = entity_ref.get::<IconImage>() {
        let textures = world.resource::<MyTextures>();
        ec.insert(icon.sprite(textures));
    }

    if let Some(physics) = entity_ref.get::<PhysicsConfig>() {
        ec.insert((
            RigidBody::Dynamic,
            Collider::cuboid(physics.shape.x, physics.shape.y),
            ColliderMassProperties::Mass(physics.mass),
        ));
    }
}

pub fn feed_to_attach(eidos: Entity, world: &World, mut ec: EntityCommands) {
    let entity_ref = world.entity(eidos);

    if let Some(icon) = entity_ref.get::<IconImage>() {
        let textures = world.resource::<MyTextures>();
        let sprite = icon.sprite(textures);
        ec.insert(sprite);

        if let Some(holds) = entity_ref.get::<HoldsConfig>() {
            ec.insert((
                bevy::sprite::Anchor(holds.get_offset()),
                crate::physics::camera::RotateWithMouse::new(holds.get_rotate_range()),
            ));
        }
    }
}

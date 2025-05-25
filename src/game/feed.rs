use super::ecs::*;
use crate::assets::RpgTextures;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn feed_to_concrete(eidos: Entity, world: &World, mut ec: EntityCommands) {
    let entity_ref = world.entity(eidos);

    if let Some(icon) = entity_ref.get::<IconImage>() {
        let rpg_folder = world.resource::<RpgTextures>();
        ec.insert(icon.sprite(rpg_folder));
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

    // #18349 not in bevy 0.16?
    let mut anchor = bevy::sprite::Anchor::Center;

    if let Some(holds) = entity_ref.get::<HoldsConfig>() {
        anchor = bevy::sprite::Anchor::Custom(holds.get_offset());
        ec.insert(crate::physics::camera::RotateWithMouse::new(
            holds.get_rotate_range(),
        ));
    }

    if let Some(icon) = entity_ref.get::<IconImage>() {
        let rpg_folder = world.resource::<RpgTextures>();
        let mut sprite = icon.sprite(rpg_folder);
        sprite.anchor = anchor;
        ec.insert(sprite);
    }
}

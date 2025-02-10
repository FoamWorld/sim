use crate::{
    constants::CHARACTER_LAYER,
    control::*,
    game::{health::*, item::*, object::*},
};
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn setup_character(mut commands: Commands, asset_server: Res<AssetServer>) {
    let launcher = commands
        .spawn((NonUnique("launcher".to_string()), Health::fragile()))
        .id();

    // two hands
    let mut storage = ItemStorage::with_capacity(2);
    storage.force_give(&mut commands, launcher);

    commands.spawn((
        Sprite::from_image(asset_server.get_handle("textures/character.png").unwrap()),
        Transform::from_xyz(0.0, 0.0, CHARACTER_LAYER),
        RigidBody::Dynamic,
        Collider::rectangle(16.0, 32.0),
        LockedAxes::ROTATION_LOCKED,
        MovementSpeed(100.0),
        Actor,
        storage,
    ));
}

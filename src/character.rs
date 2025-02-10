use bevy::prelude::*;
use avian2d::prelude::*;
use crate::{constants::CHARACTER_LAYER, control::*};

pub fn setup_character(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.get_handle("textures/character.png").unwrap()),
        Transform::from_xyz(0.0, 0.0, CHARACTER_LAYER),
        RigidBody::Dynamic,
        Collider::rectangle(16.0, 32.0),
        LockedAxes::ROTATION_LOCKED,
        MovementSpeed(100.0),
        Actor,
    ));
}

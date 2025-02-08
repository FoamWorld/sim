use crate::{constants::UNIT_PER_METER, message::Sign}; // wrong logic in fact, [todo]
use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;

fn spawn_wall(commands: &mut Commands, transform: Transform, sprite: Sprite, collider: Collider) {
    commands.spawn((sprite, transform, collider, RigidBody::Static));
}

const X_DIST: Scalar = 384.0;
const Y_DIST: Scalar = 288.0;
pub fn spawn_room(commands: &mut Commands) {
    let vertical_sprite = Sprite::from_color(
        bevy::color::palettes::basic::GRAY,
        Vec2::new(UNIT_PER_METER, 608.0),
    );
    let vertical_collider = Collider::rectangle(UNIT_PER_METER, 608.0);
    spawn_wall(
        commands,
        Transform::from_xyz(-X_DIST, 0.0, 0.0),
        vertical_sprite.clone(),
        vertical_collider.clone(),
    );
    spawn_wall(
        commands,
        Transform::from_xyz(X_DIST, 0.0, 0.0),
        vertical_sprite.clone(),
        vertical_collider.clone(),
    );

    let horizonal_sprite = Sprite::from_color(
        bevy::color::palettes::basic::GRAY,
        Vec2::new(800.0, UNIT_PER_METER),
    );
    let horizonal_collider = Collider::rectangle(800.0, UNIT_PER_METER);
    spawn_wall(
        commands,
        Transform::from_xyz(0.0, -Y_DIST, 0.0),
        horizonal_sprite.clone(),
        horizonal_collider.clone(),
    );
    spawn_wall(
        commands,
        Transform::from_xyz(0.0, Y_DIST, 0.0),
        horizonal_sprite.clone(),
        horizonal_collider.clone(),
    );
}

pub fn spawn_sign(commands: &mut Commands, x: Scalar, y: Scalar, sprite: Sprite) {
    commands.spawn((
        sprite,
        Transform::from_xyz(x, y, 0.0),
        RigidBody::Static,
        Collider::rectangle(28.0, 26.0),
        Sign("This is a sign.".to_string()),
    ));
}

use crate::constants::*;
use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;

fn spawn_wall(commands: &mut Commands, transform: Transform, sprite: Sprite, collider: Collider) {
    commands.spawn((sprite, transform, collider, RigidBody::Static));
}

const X_DIST: Scalar = (VIEWPORT_WIDTH - GRID_SIZE) * 0.5;
const Y_DIST: Scalar = (VIEWPORT_HEIGHT - GRID_SIZE) * 0.5;
pub fn spawn_room(commands: &mut Commands) {
    let vertical_sprite = Sprite::from_color(
        bevy::color::palettes::basic::GRAY,
        Vec2::new(GRID_SIZE, VIEWPORT_HEIGHT),
    );
    let vertical_collider = Collider::rectangle(GRID_SIZE, VIEWPORT_HEIGHT);
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
        Vec2::new(VIEWPORT_WIDTH, GRID_SIZE),
    );
    let horizonal_collider = Collider::rectangle(VIEWPORT_WIDTH, GRID_SIZE);
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

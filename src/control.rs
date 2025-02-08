use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

pub fn inputs_wait(mut time: ResMut<Time<Physics>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Enter) {
        time.advance_by(std::time::Duration::from_secs_f64(1.0 / 60.0));
    }
}

#[derive(Component)]
pub struct Actor;

#[derive(Component)]
pub struct MovementSpeed(pub Scalar);

pub fn inputs_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut actors: Query<(&mut LinearVelocity, &MovementSpeed), With<Actor>>,
) {
    for (mut linear_velocity, movement_speed) in &mut actors {
        let xneg = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
        let xpos = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
        let yneg = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
        let ypos = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
        linear_velocity.x = (xpos as i8 - xneg as i8) as Scalar * movement_speed.0;
        linear_velocity.y = (ypos as i8 - yneg as i8) as Scalar * movement_speed.0;
    }
}

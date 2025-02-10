use crate::state::*;
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ControlCode {
    Pause,
    Debug,
    Wait,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Use,
    // Consume,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum InputDetectionType {
    Pressed(KeyCode),
    JustPressed(KeyCode),
    EitherPressed(KeyCode, KeyCode),
    EitherJustPressed(KeyCode, KeyCode),
}

#[derive(Resource, Debug, PartialEq, Eq, Clone)]
pub struct ControlSettings(HashMap<ControlCode, InputDetectionType>);

impl Default for ControlSettings {
    fn default() -> Self {
        let list = vec![
            (
                ControlCode::Pause,
                InputDetectionType::JustPressed(KeyCode::Escape),
            ),
            (
                ControlCode::Debug,
                InputDetectionType::EitherJustPressed(KeyCode::F3, KeyCode::KeyH),
            ),
            (
                ControlCode::Wait,
                InputDetectionType::JustPressed(KeyCode::Enter),
            ),
            (
                ControlCode::MoveLeft,
                InputDetectionType::EitherPressed(KeyCode::KeyA, KeyCode::ArrowLeft),
            ),
            (
                ControlCode::MoveRight,
                InputDetectionType::EitherPressed(KeyCode::KeyD, KeyCode::ArrowRight),
            ),
            (
                ControlCode::MoveUp,
                InputDetectionType::EitherPressed(KeyCode::KeyW, KeyCode::ArrowUp),
            ),
            (
                ControlCode::MoveDown,
                InputDetectionType::EitherPressed(KeyCode::KeyS, KeyCode::ArrowDown),
            ),
            (
                ControlCode::Use,
                InputDetectionType::JustPressed(KeyCode::KeyT),
            ),
        ];
        let map: HashMap<_, _> = list.into_iter().collect();
        Self(map)
    }
}

impl ControlSettings {
    /// May support combined key inputs in the future.
    pub fn check(&self, control_code: ControlCode, input: &Res<ButtonInput<KeyCode>>) -> bool {
        if let Some(key_code) = self.0.get(&control_code) {
            match *key_code {
                InputDetectionType::Pressed(x) => input.pressed(x),
                InputDetectionType::JustPressed(x) => input.just_pressed(x),
                InputDetectionType::EitherPressed(x, y) => input.any_pressed([x, y]),
                InputDetectionType::EitherJustPressed(x, y) => input.any_just_pressed([x, y]),
            }
        } else {
            false
        }
    }
}

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ControlSettings>().add_systems(
            Update,
            (inputs_move, inputs_wait, crate::game::inputs_use).run_if(in_state(AppState::InGame)),
            // todo: add in_state(RunState::Running)
        );
    }
}

pub fn inputs_wait(
    mut time: ResMut<Time<Physics>>,
    keys: Res<ButtonInput<KeyCode>>,
    control_settings: Res<ControlSettings>,
) {
    if control_settings.check(ControlCode::Wait, &keys) {
        time.advance_by(std::time::Duration::from_secs_f64(1.0 / 60.0));
    }
}

#[derive(Component)]
pub struct Actor;

#[derive(Component)]
pub struct MovementSpeed(pub Scalar);

pub fn inputs_move(
    keys: Res<ButtonInput<KeyCode>>,
    control_settings: Res<ControlSettings>,
    mut actors: Query<(&mut LinearVelocity, &MovementSpeed), With<Actor>>,
) {
    for (mut linear_velocity, movement_speed) in &mut actors {
        let xneg = control_settings.check(ControlCode::MoveLeft, &keys);
        let xpos = control_settings.check(ControlCode::MoveRight, &keys);
        let yneg = control_settings.check(ControlCode::MoveDown, &keys);
        let ypos = control_settings.check(ControlCode::MoveUp, &keys);
        linear_velocity.x = (xpos as i8 - xneg as i8) as Scalar * movement_speed.0;
        linear_velocity.y = (ypos as i8 - yneg as i8) as Scalar * movement_speed.0;
    }
}

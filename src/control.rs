use crate::{character::*, constants::Scalar, state::*};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
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
    Activate,
    Modify,
    Throw,
    Pick,
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
                ControlCode::Activate,
                InputDetectionType::JustPressed(KeyCode::KeyT),
            ),
            (
                ControlCode::Modify,
                InputDetectionType::JustPressed(KeyCode::KeyI),
            ),
            (
                ControlCode::Throw,
                InputDetectionType::JustPressed(KeyCode::KeyQ),
            ),
            (
                ControlCode::Pick,
                InputDetectionType::JustPressed(KeyCode::KeyG),
            ),
        ];
        let map: HashMap<_, _> = list.into_iter().collect();
        Self(map)
    }
}

impl ControlSettings {
    /// May support combined key inputs in the future.
    pub fn check(&self, control_code: ControlCode, input: &ButtonInput<KeyCode>) -> bool {
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
        app.init_resource::<ControlSettings>();
        app.add_systems(Update, (inputs_move, inputs_wait).in_set(InGameSet::Input));
        app.add_systems(FixedUpdate, change_facing.in_set(InGameSet::PostInput));
    }
}

pub fn inputs_wait(
    mut time: ResMut<Time<Fixed>>,
    keys: Res<ButtonInput<KeyCode>>,
    control_settings: Res<ControlSettings>,
) {
    if control_settings.check(ControlCode::Wait, &keys) {
        time.advance_by(std::time::Duration::from_secs_f64(1.0 / 60.0));
    }
}

#[derive(Component)]
pub struct MovementSpeed(pub Scalar);

pub fn change_facing(mut actors: Query<(&mut Actor, &mut Sprite), Changed<Actor>>) {
    if let Ok((actor, mut sprite)) = actors.single_mut() {
        sprite.flip_x = match actor.0 {
            ActorFacing::Left => true,
            ActorFacing::Right => false,
        }
    }
}

pub fn inputs_move(
    keys: Res<ButtonInput<KeyCode>>,
    control_settings: Res<ControlSettings>,
    mut actors: Query<(&mut Velocity, &mut Actor, &MovementSpeed)>,
) {
    if let Ok((mut velocity, mut actor, movement_speed)) = actors.single_mut() {
        let to_left = control_settings.check(ControlCode::MoveLeft, &keys);
        let to_right = control_settings.check(ControlCode::MoveRight, &keys);

        if to_left {
            actor.set_facing(ActorFacing::Left);
            velocity.linvel.x = -movement_speed.0;
        } else if to_right {
            actor.set_facing(ActorFacing::Right);
            velocity.linvel.x = movement_speed.0;
        }

        // let yneg = control_settings.check(ControlCode::MoveDown, &keys);
        let jump = control_settings.check(ControlCode::MoveUp, &keys);
        if velocity.linvel.y.abs() < 0.1 && jump {
            velocity.linvel.y = 120.0;
        }
    }
}

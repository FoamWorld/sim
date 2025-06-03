use crate::{character::*, constants::*, state::*};
use bevy::prelude::*;
use bevy_tnua::prelude::*;
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

#[derive(Component)]
pub struct MoveDownTimer(pub Timer);

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ControlSettings>();

        app.add_systems(Update, (inputs_move, inputs_wait).in_set(InGameSet::Input));
        app.add_systems(FixedUpdate, change_facing.in_set(InGameSet::PostInput));
        app.add_systems(
            FixedUpdate,
            (|time: Res<Time>, mut timers: Query<&mut MoveDownTimer>| {
                for mut move_down in timers.iter_mut() {
                    move_down.0.tick(time.delta());
                }
            })
            .in_set(InGameSet::Logic),
        );
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
    mut actors: Query<(&mut Actor, &mut TnuaController, &mut MoveDownTimer)>,
) {
    if let Ok((mut actor, mut controller, mut move_down)) = actors.single_mut() {
        let to_left = control_settings.check(ControlCode::MoveLeft, &keys);
        let to_right = control_settings.check(ControlCode::MoveRight, &keys);
        let jump = control_settings.check(ControlCode::MoveUp, &keys);
        let drop = control_settings.check(ControlCode::MoveDown, &keys);

        let mut direction = Vec2::ZERO;

        if to_left {
            actor.set_facing(ActorFacing::Left);
            direction -= Vec2::X;
        } else if to_right {
            actor.set_facing(ActorFacing::Right);
            direction += Vec2::X;
        }

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: (direction * 120.0).extend(0.0),
            float_height: CHARACTER_Y_LENGTH * 0.5 + 2.0,
            ..default()
        });

        if jump {
            controller.action(TnuaBuiltinJump {
                height: 40.0,
                ..default()
            });
        } else if drop {
            move_down.0.reset();
        }
    }
}

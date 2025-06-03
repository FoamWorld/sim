use crate::{assets::RpgTextures, constants::*, control::*, state::WillRemove};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// For all entities that interact with physics like the player.
#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct IsActive;

#[derive(Resource, Default)]
pub struct ActorPosition {
    pub facing_right: bool,
    pub center: Vec2,
    pub facing_offset: Vec2,
    pub primary_hand_offset: Vec2,
    // pub secondary_hand: Vec2,
}

#[derive(Reflect, PartialEq)]
pub enum ActorFacing {
    Left,
    Right,
}

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Actor(pub ActorFacing);

impl Actor {
    pub fn set_facing(&mut self, facing: ActorFacing) {
        self.0 = facing;
    }

    pub fn get_facing_offset(&self) -> Vec2 {
        match self.0 {
            ActorFacing::Left => Vec2::new(-1.0, 0.0),
            ActorFacing::Right => Vec2::new(1.0, 0.0),
        }
    }

    pub fn get_primary_hand_offset(&self) -> Vec2 {
        match self.0 {
            ActorFacing::Left => Vec2::new(-11.0, -6.0),
            ActorFacing::Right => Vec2::new(11.0, -6.0),
        }
    }
}

pub fn setup_character(mut commands: Commands, rpg_folder: Res<RpgTextures>) {
    let mut timer = Timer::from_seconds(0.1, TimerMode::Once);
    timer.set_elapsed(timer.duration());
    commands.spawn((
        Sprite {
            image: rpg_folder.get_image_handle("character"),
            custom_size: Some(Vec2::new(CHARACTER_X_LENGTH, CHARACTER_Y_LENGTH)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, CHARACTER_LAYER),
        RigidBody::Dynamic,
        Collider::capsule_y(
            CHARACTER_X_LENGTH * 0.5,
            (CHARACTER_Y_LENGTH - CHARACTER_X_LENGTH) * 0.5,
        ),
        ColliderMassProperties::Mass(70.0),
        LockedAxes::ROTATION_LOCKED,
        MoveDownTimer(timer),
        Actor(ActorFacing::Right),
        Character,
        bevy_tnua::prelude::TnuaController::default(),
        bevy_tnua_rapier2d::TnuaRapier2dSensorShape(Collider::cuboid(
            CHARACTER_X_LENGTH * 0.5,
            1.0,
        )),
        bevy_tnua::TnuaGhostSensor::default(),
        WillRemove,
    ));
}

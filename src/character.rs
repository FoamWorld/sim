use crate::{assets::MyTextures, constants::*, control::*, markers::ObjRoot};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// For all entities that interact with physics like the player.
#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct IsActive;

#[derive(Reflect, Default, Clone, Copy, PartialEq)]
pub enum ActorFacing {
    Left,
    #[default]
    Right,
}

impl ActorFacing {
    pub fn get_facing_offset(&self) -> Vec2 {
        match self {
            ActorFacing::Left => Vec2::new(-1.0, 0.0),
            ActorFacing::Right => Vec2::new(1.0, 0.0),
        }
    }

    pub fn get_primary_hand_offset(&self) -> Vec2 {
        match self {
            ActorFacing::Left => Vec2::new(-11.0, -6.0),
            ActorFacing::Right => Vec2::new(11.0, -6.0),
        }
    }
}

#[derive(Resource, Default)]
pub struct ActorStatus {
    pub entity: Option<Entity>,
    pub facing: ActorFacing,
    pub center: Vec2,
    pub facing_offset: Vec2,
    pub primary_hand_offset: Vec2,
    // pub secondary_hand: Vec2,
}

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Actor(pub ActorFacing);

pub fn setup_character(
    mut commands: Commands,
    textures: Res<MyTextures>,
    mut status: ResMut<ActorStatus>,
) {
    let mut ec = commands.spawn((Actor(ActorFacing::Right), Character, ObjRoot));

    let mut timer = Timer::from_seconds(0.1, TimerMode::Once);
    timer.set_elapsed(timer.duration());
    ec.insert((
        Sprite {
            image: textures.get_image_handle("character"),
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
        bevy_tnua::prelude::TnuaController::default(),
        bevy_tnua_rapier2d::TnuaRapier2dSensorShape(Collider::cuboid(
            CHARACTER_X_LENGTH * 0.5,
            1.0,
        )),
        bevy_tnua::TnuaGhostSensor::default(),
        SolverGroups::new(
            Group::GROUP_1 | Group::GROUP_2,
            Group::GROUP_1 | Group::GROUP_2,
        ),
    ));
    status.entity = Some(ec.id());
}

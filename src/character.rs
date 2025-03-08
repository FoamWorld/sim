use crate::{assets::RpgTextures, constants::*, control::*, state::WillRemove};
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct IsActive;

#[derive(Resource, Default)]
pub struct ActorPosition {
    pub center: Vec2,
    pub primary_hand_offset: Vec2,
    // pub secondary_hand: Vec2,
}

#[derive(Reflect)]
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

    pub fn get_primary_hand_offset(&self) -> Vec2 {
        match self.0 {
            ActorFacing::Left => Vec2::new(-11.0, -6.0),
            ActorFacing::Right => Vec2::new(11.0, -6.0),
        }
    }
}

pub fn setup_character(mut commands: Commands, rpg_folder: Res<RpgTextures>) {
    commands.spawn((
        Sprite {
            image: rpg_folder.get_image_handle("character"),
            custom_size: Some(Vec2::new(CHARACTER_X_LENGTH, CHARACTER_Y_LENGTH)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, CHARACTER_LAYER),
        RigidBody::Dynamic,
        Collider::rectangle(CHARACTER_X_LENGTH, CHARACTER_Y_LENGTH),
        LockedAxes::ROTATION_LOCKED,
        Mass(70.0),
        MovementSpeed(100.0),
        Actor(ActorFacing::Right),
        WillRemove,
    ));
}

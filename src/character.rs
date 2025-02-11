use crate::{
    assets::RpgTextures,
    constants::*,
    control::*,
    game::{health::*, item::*, object::*},
    physics::camera::RotateWithMouse,
};
use avian2d::prelude::*;
use bevy::{prelude::*, sprite::Anchor};

#[derive(Resource, Default)]
pub struct SelectedSlot(pub usize);

pub fn setup_character(
    mut commands: Commands,
    world: &World,
    // asset_server: Res<AssetServer>,
    rpg_folder: Res<RpgTextures>,
) {
    let launcher = commands
        .spawn((NonUnique("launcher".to_string()), Health::fragile()))
        .id();

    // two hands
    let mut storage = ItemStorage::with_capacity(2);
    storage.force_give(&mut commands, launcher);

    let mut actor = commands.spawn((
        Sprite::from_image(rpg_folder.get_image_handle("character")),
        Transform::from_xyz(0.0, 0.0, CHARACTER_LAYER),
        RigidBody::Dynamic,
        Collider::rectangle(16.0, 32.0),
        LockedAxes::ROTATION_LOCKED,
        MovementSpeed(100.0),
        Actor,
        storage.clone(),
    ));

    // selected
    let chosen = world.get_resource::<SelectedSlot>().unwrap().0;
    if let Some(item) = storage.get_index(chosen) {
        actor.with_children(|parent: &mut ChildBuilder<'_>| {
            setup_item_sprite(item, parent, &rpg_folder);
        });
    };
}

pub fn setup_item_sprite(_item: Entity, parent: &mut ChildBuilder, rpg_folder: &Res<RpgTextures>) {
    parent.spawn((
        Sprite {
            image: rpg_folder.get_image_handle("items"),
            texture_atlas: Some(rpg_folder.get_texture_atlas("items", 0)),
            anchor: Anchor::Custom(Vec2::new(-0.4, -0.4)),
            ..default()
        },
        Transform::from_translation(CHARACTER_LEFT_HAND_OFFSET.extend(CHARACTER_HOLD_OFFSET)),
        RotateWithMouse(Rotation::FRAC_PI_4),
    ));
}

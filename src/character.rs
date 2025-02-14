use crate::{
    assets::RpgTextures,
    constants::*,
    control::*,
    game::{
        item::{debug_wand::DebugWand, *},
        object::*,
    },
};
use avian2d::prelude::*;
use bevy::prelude::*;
use std::any::Any;

#[derive(Resource, Default)]
pub struct SelectedSlot(pub usize);

#[derive(Component)]
pub struct IsSelected;

pub fn setup_character(
    mut commands: Commands,
    world: &World,
    // asset_server: Res<AssetServer>,
    rpg_folder: Res<RpgTextures>,
) {
    let debug_wand = DebugWand { mode: 3 };
    let ty = IsObject(debug_wand.type_id());
    let ty2 = IsItem(debug_wand.type_id());
    let launcher = commands.spawn((debug_wand, ty.clone(), ty2)).id();
    ty.spawn_into(world, &mut commands, launcher, &rpg_folder);

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
    let ec = parent.spawn(Transform::from_translation(
        CHARACTER_LEFT_HAND_OFFSET.extend(CHARACTER_HOLD_OFFSET),
    ));
}

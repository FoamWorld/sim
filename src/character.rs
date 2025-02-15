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
pub struct IsActive;

pub fn setup_character(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    rpg_folder: Res<RpgTextures>,
) {
    let launcher = {
        let debug_wand = DebugWand { mode: 3 };
        let ty = IsObject(debug_wand.type_id());
        let ty2 = IsItem(debug_wand.type_id());
        let mut ec = commands.spawn((
            debug_wand,
            ty.clone(),
            ty2,
            Transform::from_xyz(0.0, -100.0, 15.0),
        ));
        debug_wand.add_visual_components(&mut ec, &rpg_folder);
        ec.id()
    };

    let mut storage = ItemStorage::with_capacity(2);
    storage.force_give(&mut commands, launcher);

    commands.spawn((
        Sprite::from_image(rpg_folder.get_image_handle("character")),
        Transform::from_xyz(0.0, 0.0, CHARACTER_LAYER),
        RigidBody::Dynamic,
        Collider::rectangle(16.0, 32.0),
        LockedAxes::ROTATION_LOCKED,
        MovementSpeed(100.0),
        Actor,
        storage,
    ));
}

pub fn setup_attached_image(
    mut commands: Commands,
    world: &World,
    // asset_server: Res<AssetServer>,
    actors: Query<Entity, With<Actor>>,
    q_st: Query<&ItemStorage>,
    q_obj: Query<&IsObject>,
) {
    let actor = actors.single();
    let storage = q_st.get(actor).unwrap();
    let chosen = world.get_resource::<SelectedSlot>().unwrap().0;
    if let Some(item) = storage.get_index(chosen) {
        commands
            .entity(actor)
            .with_children(|parent: &mut ChildBuilder<'_>| {
                setup_item_sprite(world, item, parent, q_obj);
            });
    };
}

pub fn setup_item_sprite(
    world: &World,
    item: Entity,
    parent: &mut ChildBuilder,
    q_obj: Query<&IsObject>,
) {
    let registry = world.get_resource::<AppTypeRegistry>().unwrap();
    let it = q_obj.get(item).unwrap();
    let mut ec = parent.spawn((
        Transform::from_translation(CHARACTER_LEFT_HAND_OFFSET.extend(CHARACTER_HOLD_OFFSET)),
        IsActive,
    ));
    it.add_components(
        world,
        &mut ec,
        item,
        registry,
        AdditionConfig::CHARACTER_ATTACH,
    );
}

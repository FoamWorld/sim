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
pub struct Inventory;

#[derive(Component)]
pub struct IsActive;

pub fn setup_character(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    rpg_folder: Res<RpgTextures>,
) {
    let launcher = {
        let debug_wand = DebugWand { mode: 3 };
        commands
            .spawn((
                debug_wand,
                IsObject(debug_wand.type_id()),
                IsItem(debug_wand.type_id()),
            ))
            .id()
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

pub fn setup_inventory(mut commands: Commands, world: &World, actors: Query<Entity, With<Actor>>) {
    let type_registry = world.get_resource::<AppTypeRegistry>().unwrap();
    let actor = actors.single();
    let storage = world.entity(actor).get::<ItemStorage>().unwrap();
    let size = storage.size();
    let mut inventory = commands.spawn((
        Inventory,
        Node {
            position_type: PositionType::Absolute,
            height: Val::Px(64.0),
            bottom: Val::Px(16.0),
            justify_self: JustifySelf::Center,
            justify_items: JustifyItems::Center,
            flex_direction: FlexDirection::Row,
            ..default()
        },
    ));
    inventory.with_children(|builder| {
        for ind in 0..size {
            let mut ec = builder.spawn((
                Node {
                    width: Val::Percent(64.0),
                    height: Val::Percent(64.0),
                    margin: UiRect::horizontal(Val::Px(4.0)),
                    ..default()
                },
                Outline::new(Val::Px(1.0), Val::ZERO, Color::WHITE),
                BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
            ));
            if let Some(item) = storage.storage[ind] {
                let object = world.entity(item).get::<IsObject>().unwrap();
                object.add_components(world, &mut ec, item, type_registry, AdditionConfig::IN_GRID);
            } else {
                ec.insert(ImageNode::solid_color(Color::NONE));
            }
        }
    });
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

use super::{ecs::*, item::wand::WandModel, item_storage::*};
use crate::{assets::RpgTextures, character::*, constants::*, state::WillRemove};
use bevy::prelude::*;

#[derive(Resource)]
pub struct Inventory {
    pub size: usize,
    pub selected: usize,
    pub bind: Option<Entity>,
}

#[derive(Event)]
pub struct InventorySelectedUpdateEvent;

#[derive(Component)]
pub struct UiGrid(pub usize);

pub fn setup_inventory(mut commands: Commands, mut inventory: ResMut<Inventory>) {
    let launcher = commands.spawn(WandModel { mode: 3 }).id();

    let mut storage = ItemStorage::with_capacity(4);
    storage.force_give(0, launcher);
    inventory.size = 4;
    inventory.bind = Some(commands.spawn((storage, WillRemove)).id());
}

pub fn setup_inventory_ui(mut commands: Commands, world: &World, inventory: Res<Inventory>) {
    let storage = world
        .entity(inventory.bind.unwrap())
        .get::<ItemStorage>()
        .unwrap();
    let size = inventory.size;
    let mut ui = commands.spawn((Node {
        position_type: PositionType::Absolute,
        height: Val::Px(GRID_SIZE),
        bottom: Val::Px(4.0),
        justify_self: JustifySelf::Center,
        justify_items: JustifyItems::Center,
        flex_direction: FlexDirection::Row,
        ..default()
    },));
    ui.with_children(|builder| {
        for ind in 0..size {
            let mut ec = builder.spawn((
                Node {
                    width: Val::Px(GRID_SIZE),
                    height: Val::Px(GRID_SIZE),
                    margin: UiRect::horizontal(Val::Px(4.0)),
                    ..default()
                },
                Outline::new(Val::Px(1.0), Val::ZERO, Color::BLACK),
                UiGrid(ind),
                WillRemove,
            ));
            if let Some(item) = storage.storage[ind] {
                if let Some(icon) = world.entity(item).get::<IconImage>() {
                    icon.inserts_image(&mut ec, world.resource::<RpgTextures>());
                } else {
                    ec.insert(ImageNode::solid_color(Color::BLACK));
                };
            } else {
                ec.insert(ImageNode::solid_color(Color::NONE));
            }
        }
    });
}

pub fn move_outline(inventory: Res<Inventory>, mut query_grid: Query<(&UiGrid, &mut Outline)>) {
    for (grid, mut outline) in query_grid.iter_mut() {
        if grid.0 == inventory.selected {
            outline.color = Color::WHITE;
        } else {
            outline.color = Color::BLACK;
        }
    }
}

pub fn update_attached_image(
    mut commands: Commands,
    world: &World,
    inventory: Res<Inventory>,
    actors: Query<Entity, With<Actor>>,
    mut reader: EventReader<InventorySelectedUpdateEvent>,
    position: Res<ActorPosition>,
) {
    if reader.is_empty() {
        return;
    }
    reader.clear();

    let actor = actors.single().unwrap();
    commands.entity(actor).despawn_related::<Children>();

    let inv = inventory.bind.unwrap();
    let storage = world.entity(inv).get::<ItemStorage>().unwrap();
    if let Some(item) = storage.get_index(inventory.selected) {
        commands
            .entity(actor)
            .with_children(|parent: &mut ChildSpawnerCommands| {
                setup_item_sprite(world, item, parent, position);
            });
    };
}

fn setup_item_sprite(
    world: &World,
    item: Entity,
    parent: &mut ChildSpawnerCommands,
    position: Res<ActorPosition>,
) {
    let rpg_folder = world.resource::<RpgTextures>();
    let mut commands = parent.spawn((
        Transform::from_translation(position.primary_hand_offset.extend(1.0)),
        IsActive,
    ));
    if let Some(icon) = world.entity(item).get::<IconImage>() {
        icon.inserts_sprite(&mut commands, rpg_folder);
    }
}

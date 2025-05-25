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
pub struct InventoryGrid(pub usize);

pub fn setup_inventory(mut commands: Commands, mut inventory: ResMut<Inventory>) {
    let launcher = commands.spawn(WandModel { mode: 3 }).id();

    let storage = commands.spawn(ItemStorage::with_capacity(4)).id();
    inventory.size = 4;
    inventory.bind = Some(storage);

    commands
        .entity(storage)
        .queue(move |mut entity_world_mut: EntityWorldMut| {
            entity_world_mut
                .get_mut::<ItemStorage>()
                .unwrap()
                .force_give(0, launcher, 4.0);
        });
}

pub fn setup_inventory_ui(mut commands: Commands, inventory: Res<Inventory>) {
    let size = inventory.size;
    let ui = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                height: Val::Px(GRID_SIZE),
                bottom: Val::Px(4.0),
                justify_self: JustifySelf::Center,
                justify_items: JustifyItems::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            WillRemove,
        ))
        .id();
    for ind in 0..size {
        commands.spawn((
            ChildOf(ui),
            Node {
                width: Val::Px(GRID_SIZE),
                height: Val::Px(GRID_SIZE),
                margin: UiRect::horizontal(Val::Px(4.0)),
                ..default()
            },
            Outline::new(Val::Px(1.0), Val::ZERO, Color::BLACK),
            InventoryGrid(ind),
        ));
    }
}

pub fn update_grid_images(
    mut commands: Commands,
    world: &World,
    query: Query<(Entity, &InventoryGrid)>,
    item_storage: Query<&ItemStorage, Changed<ItemStorage>>,
) {
    if let Ok(storage) = item_storage.single() {
        for (entity, grid) in query {
            let mut ec = commands.entity(entity);
            if let Some(item) = storage.storage[grid.0] {
                if let Some(icon) = world.entity(item).get::<IconImage>() {
                    let rpg_folder = world.resource::<RpgTextures>();
                    icon.inserts_image(&mut ec, rpg_folder);
                } else {
                    ec.insert(ImageNode::solid_color(Color::BLACK));
                };
            } else {
                ec.insert(ImageNode::solid_color(Color::NONE));
            }
        }
    }
}

pub fn move_outline(
    inventory: Res<Inventory>,
    mut query_grid: Query<(&InventoryGrid, &mut Outline)>,
) {
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
        let ec = commands.spawn((
            Methexis(item),
            ChildOf(actor),
            Transform::from_translation(position.primary_hand_offset.extend(1.0)),
            IsActive,
        ));

        super::feed::feed_to_attach(item, world, ec);
    };
}

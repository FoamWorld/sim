use crate::{assets::RpgTextures, character::*, control::*, scene::*, state::*};
use avian2d::prelude::*;
use bevy::prelude::*;

use item::*;

pub mod character;
use character::*;

pub mod inventory;
use inventory::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Inventory {
            size: 0,
            selected: 0,
            bind: None,
        })
        .init_resource::<ActorPosition>();

        app.add_event::<InventorySelectedUpdateEvent>()
            .add_event::<mob::health::HealthClearedEvent>();

        app.add_systems(
            OnEnter(ProcessState::PreEnterGame),
            (
                setup_inventory,
                setup_inventory_ui,
                |mut writer: EventWriter<InventorySelectedUpdateEvent>| {
                    writer.send(InventorySelectedUpdateEvent);
                },
            )
                .chain()
                .in_set(ProcessSet::Late),
        );

        app.add_systems(
            Update,
            (
                inputs_use,
                inputs_modify,
                inputs_throw,
                inputs_number,
                // add input handling here
            )
                .in_set(InGameSet::Input),
        );

        app.add_systems(
            Update,
            (move_outline, update_attached_image).in_set(InGameSet::PostInput),
        );

        app.add_systems(
            FixedUpdate,
            (update_actor_position, mob::health::read_health_cleared).in_set(InGameSet::Logic),
        );
    }
}

pub mod item;
pub mod mob;
pub mod object;

/* Handling inputs. */

fn inputs_use(
    mut commands: Commands,
    world: &World,
    keys: Res<ButtonInput<KeyCode>>,
    click: Res<ButtonInput<MouseButton>>,
    control_settings: Res<ControlSettings>,
    inventory: Res<Inventory>,
    position: Res<ActorPosition>,
    coords: Res<crate::physics::camera::CursorCoords>,
    rpg_folder: Res<RpgTextures>,
) {
    if click.just_pressed(MouseButton::Left) || control_settings.check(ControlCode::Use, &keys) {
        let inv = inventory.bind.unwrap();
        let index = inventory.selected;
        reach_inventory_item_then(world, inv, index, |guarded, item| {
            if guarded.check_can_use() {
                guarded.item_use(
                    &mut commands,
                    item,
                    position.center + position.primary_hand_offset,
                    coords.0,
                    rpg_folder.as_ref(),
                );
            }
        });
    }
}

fn inputs_modify(
    mut commands: Commands,
    world: &World,
    keys: Res<ButtonInput<KeyCode>>,
    click: Res<ButtonInput<MouseButton>>,
    control_settings: Res<ControlSettings>,
    inventory: Res<Inventory>,
) {
    if click.just_pressed(MouseButton::Right) || control_settings.check(ControlCode::Modify, &keys)
    {
        let inv = inventory.bind.unwrap();
        let index = inventory.selected;
        reach_inventory_item_then(world, inv, index, |guarded, item| {
            if guarded.check_can_modify() {
                guarded.item_modify(&mut commands, item);
            }
        });
    }
}

fn inputs_throw(
    mut commands: Commands,
    world: &World,
    keys: Res<ButtonInput<KeyCode>>,
    control_settings: Res<ControlSettings>,
    inventory: Res<Inventory>,
) {
    if control_settings.check(ControlCode::Throw, &keys) {
        let inv = inventory.bind.unwrap();
        let storage = world.entity(inv).get::<ItemStorage>().unwrap();
        let index = inventory.selected;
        if storage.view_count(index) < 1.0 {
            return;
        }
        commands
            .entity(inv)
            .queue(move |mut entity: EntityWorldMut| {
                let mut storage = entity.get_mut::<ItemStorage>().unwrap();
                storage.extract_one(index.clone());
            });
        commands.spawn((
            // clone item here
            RigidBody::Dynamic,
        ));
    }
}

fn reach_inventory_item_then<F>(world: &World, inv: Entity, index: usize, f: F)
where
    F: FnOnce(&dyn Item, Entity) -> (),
{
    let storage = world.entity(inv).get::<ItemStorage>().unwrap();
    let item = if let Some(item) = storage.get_index(index) {
        item
    } else {
        return;
    };

    let type_registry = world.resource::<AppTypeRegistry>();
    IsItem::inspect_then(world, item, type_registry, |guarded| {
        f(guarded, item);
    });
}

fn inputs_number(
    keys: Res<ButtonInput<KeyCode>>,
    mut inventory: ResMut<Inventory>,
    mut writer: EventWriter<InventorySelectedUpdateEvent>,
) {
    let number: usize = if keys.pressed(KeyCode::Digit0) {
        0
    } else if keys.pressed(KeyCode::Digit1) {
        1
    } else if keys.pressed(KeyCode::Digit2) {
        2
    } else if keys.pressed(KeyCode::Digit3) {
        3
    } else if keys.pressed(KeyCode::Digit4) {
        4
    } else if keys.pressed(KeyCode::Digit5) {
        5
    } else if keys.pressed(KeyCode::Digit6) {
        6
    } else if keys.pressed(KeyCode::Digit7) {
        7
    } else if keys.pressed(KeyCode::Digit8) {
        8
    } else if keys.pressed(KeyCode::Digit9) {
        9
    } else {
        return;
    };
    if number < inventory.size {
        inventory.selected = number;
        writer.send(InventorySelectedUpdateEvent);
    }
}

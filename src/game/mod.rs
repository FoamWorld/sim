use crate::{character::*, control::*, physics::picking::*, scene::*, state::*};
use bevy::prelude::*;

pub mod character;
pub mod ecs;
pub mod feed;
pub mod inventory;
pub mod item;
pub mod item_control;
pub mod item_storage;
pub mod mob;
pub mod object;
pub mod summon;

use character::*;
use inventory::*;
use item_control::*;
use item_storage::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Inventory {
            size: 0,
            selected: 0,
            bind: None,
        });

        app.add_event::<InventorySelectedUpdateEvent>()
            .add_event::<mob::health::HealthClearedEvent>();

        app.add_systems(
            OnEnter(ProcessState::PreEnterGame),
            (setup_inventory, setup_inventory_ui)
                .chain()
                .in_set(ProcessSet::Late),
        );

        app.add_systems(
            FixedUpdate,
            (
                object::barrier::setup_barrier_model,
                object::barrier::setup_platform_room_model,
                object::cloth::setup_cloth_model,
                object::platform::setup_shelf_model,
                item::wand::setup_wand_model,
            )
                .in_set(InGameSet::Logic),
        );

        app.add_systems(
            Update,
            (
                detect_input_use,
                detect_input_modify,
                detect_input_throw,
                detect_input_choose,
                detect_input_pick,
            )
                .in_set(InGameSet::Input),
        );

        app.add_systems(
            Update,
            (move_outline, update_grid_images).in_set(InGameSet::PostInput),
        );

        app.add_systems(
            FixedUpdate,
            (
                update_actor_status,
                mob::health::read_health_cleared,
                |query: Query<(), Changed<ItemStorage>>,
                 mut writer: EventWriter<InventorySelectedUpdateEvent>| {
                    if !query.is_empty() {
                        writer.write(InventorySelectedUpdateEvent);
                    }
                },
            )
                .in_set(InGameSet::Logic),
        );

        app.add_systems(Update, update_attached_image.in_set(InGameSet::Ui));
    }
}

/* Handling inputs. */

fn detect_input_use(
    commands: Commands,
    world: &World,
    keys: Res<ButtonInput<KeyCode>>,
    click: Res<ButtonInput<MouseButton>>,
    control_settings: Res<ControlSettings>,
    inventory: Res<Inventory>,
    transform: Query<&GlobalTransform, With<IsActive>>,
) {
    if click.just_pressed(MouseButton::Left) || control_settings.check(ControlCode::Activate, &keys)
    {
        item_use(commands, world, inventory, transform.single().unwrap());
    }
}

fn detect_input_modify(
    commands: Commands,
    world: &World,
    keys: Res<ButtonInput<KeyCode>>,
    click: Res<ButtonInput<MouseButton>>,
    control_settings: Res<ControlSettings>,
    inventory: Res<Inventory>,
) {
    if click.just_pressed(MouseButton::Right) || control_settings.check(ControlCode::Modify, &keys)
    {
        item_modify(commands, world, inventory);
    }
}

fn detect_input_throw(
    commands: Commands,
    world: &World,
    keys: Res<ButtonInput<KeyCode>>,
    control_settings: Res<ControlSettings>,
    inventory: Res<Inventory>,
) {
    if control_settings.check(ControlCode::Throw, &keys) {
        item_throw(commands, world, inventory);
    }
}

fn detect_input_choose(
    keys: Res<ButtonInput<KeyCode>>,
    scroll: Res<bevy::input::mouse::AccumulatedMouseScroll>,
    mut inventory: ResMut<Inventory>,
    mut writer: EventWriter<InventorySelectedUpdateEvent>,
) {
    let sz = inventory.size;
    let cu = inventory.selected;
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
    } else if scroll.delta.y > 0.0 {
        if cu == 0 {
            return;
        } else {
            cu - 1
        }
    } else if scroll.delta.y < 0.0 {
        if cu == sz - 1 {
            return;
        } else {
            cu + 1
        }
    } else {
        return;
    };
    if number < sz {
        inventory.selected = number;
        writer.write(InventorySelectedUpdateEvent);
    }
}

fn detect_input_pick(
    commands: Commands,
    world: &World,
    keys: Res<ButtonInput<KeyCode>>,
    control_settings: Res<ControlSettings>,
    hover: Res<HoverEntity>,
    inventory: Res<Inventory>,
) {
    if control_settings.check(ControlCode::Pick, &keys) && hover.is_legal {
        item_pick(commands, world, hover.closest.unwrap(), inventory);
    }
}

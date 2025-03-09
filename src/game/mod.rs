use crate::{character::*, control::*, scene::*, state::*};
use bevy::prelude::*;

pub mod character;
pub mod inventory;
pub mod item;
pub mod item_control;
pub mod mob;
pub mod object;

use character::*;
use inventory::*;
use item_control::*;

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
                detect_input_use,
                detect_input_modify,
                detect_input_throw,
                detect_input_choose,
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

/* Handling inputs. */

fn detect_input_use(
    commands: Commands,
    world: &World,
    keys: Res<ButtonInput<KeyCode>>,
    click: Res<ButtonInput<MouseButton>>,
    control_settings: Res<ControlSettings>,
    inventory: Res<Inventory>,
) {
    if click.just_pressed(MouseButton::Left) || control_settings.check(ControlCode::Use, &keys) {
        item_use(commands, world, inventory);
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

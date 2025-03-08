use crate::{assets::RpgTextures, character::*, constants::*, control::*, state::*};
use avian2d::prelude::*;
use bevy::prelude::*;
use item::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<mob::health::HealthClearedEvent>();

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
            FixedUpdate,
            mob::health::read_health_cleared.run_if(in_state(AppState::InGame)),
        );
    }
}

pub mod item;
pub mod mob;
pub mod object;

fn inputs_use(
    mut commands: Commands,
    world: &World,
    keys: Res<ButtonInput<KeyCode>>,
    click: Res<ButtonInput<MouseButton>>,
    control_settings: Res<ControlSettings>,
    selected: Res<SelectedSlot>,
    coords: Res<crate::physics::camera::CursorCoords>,
    actors: Query<Entity, With<Actor>>,
    rpg_folder: Res<RpgTextures>,
) {
    if click.just_pressed(MouseButton::Left) || control_settings.check(ControlCode::Use, &keys) {
        let actor = actors.single();
        let index = selected.0;
        reach_inventory_item_then(world, actor, index, |guarded, item| {
            if guarded.check_can_use() {
                let transform = world.entity(actor).get::<Transform>().unwrap();
                let hold_point = transform.translation.truncate() + CHARACTER_LEFT_HAND_OFFSET;
                guarded.item_use(
                    &mut commands,
                    item,
                    hold_point,
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
    selected: Res<SelectedSlot>,
    actors: Query<Entity, With<Actor>>,
) {
    if click.just_pressed(MouseButton::Right) || control_settings.check(ControlCode::Modify, &keys)
    {
        let actor = actors.single();
        let index = selected.0;
        reach_inventory_item_then(world, actor, index, |guarded, item| {
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
    selected: Res<SelectedSlot>,
    actors: Query<Entity, With<Actor>>,
) {
    if control_settings.check(ControlCode::Throw, &keys) {
        let actor = actors.single();
        let storage = world.entity(actor).get::<ItemStorage>().unwrap();
        let index = selected.0;
        if storage.view_count(index) < 1.0 {
            return;
        }
        commands
            .entity(actor)
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

fn reach_inventory_item_then<F>(world: &World, actor: Entity, index: usize, f: F)
where
    F: FnOnce(&dyn Item, Entity) -> (),
{
    let storage = world.entity(actor).get::<ItemStorage>().unwrap();
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

fn inputs_number(keys: Res<ButtonInput<KeyCode>>, mut selected: ResMut<SelectedSlot>) {
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
    if number < 4 {
        selected.0 = number;
    }
}

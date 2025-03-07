use crate::{assets::RpgTextures, character::*, constants::*, control::*, state::*};
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

pub fn inputs_use(
    mut commands: Commands,
    world: &World,
    keys: Res<ButtonInput<KeyCode>>,
    click: Res<ButtonInput<MouseButton>>,
    control_settings: Res<ControlSettings>,
    coords: Res<crate::physics::camera::CursorCoords>,
    actors: Query<Entity, With<Actor>>,
    rpg_folder: Res<RpgTextures>,
) {
    let actor = actors.single();
    if click.just_pressed(MouseButton::Left) || control_settings.check(ControlCode::Use, &keys) {
        reach_inventory_item_then(world, actor, 0, |guarded, item| {
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

pub fn inputs_modify(
    mut commands: Commands,
    world: &World,
    keys: Res<ButtonInput<KeyCode>>,
    click: Res<ButtonInput<MouseButton>>,
    control_settings: Res<ControlSettings>,
    actors: Query<Entity, With<Actor>>,
) {
    let actor = actors.single();
    if click.just_pressed(MouseButton::Right) || control_settings.check(ControlCode::Modify, &keys)
    {
        reach_inventory_item_then(world, actor, 0, |guarded, item| {
            if guarded.check_can_modify() {
                guarded.item_modify(&mut commands, item);
            }
        });
    }
}

pub fn reach_inventory_item_then<F>(world: &World, actor: Entity, ind: usize, f: F)
where
    F: FnOnce(&dyn Item, Entity) -> (),
{
    let storage = world.entity(actor).get::<ItemStorage>().unwrap();
    let item = if let Some(item) = storage.get_index(ind) {
        item
    } else {
        return;
    };

    let type_registry = world.get_resource::<AppTypeRegistry>().unwrap();
    IsItem::inspect_then(world, item, type_registry, |guarded| {
        f(guarded, item);
    });
}

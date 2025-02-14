use crate::{assets::RpgTextures, constants::*, control::*};
use bevy::prelude::*;
use item::*;

pub mod health;
pub mod item;
pub mod object;

pub fn inputs_use(
    mut commands: Commands,
    world: &World,
    keys: Res<ButtonInput<KeyCode>>,
    click: Res<ButtonInput<MouseButton>>,
    control_settings: Res<ControlSettings>,
    coords: Res<crate::physics::camera::CursorCoords>,
    actors: Query<Entity, With<Actor>>,
    q_st: Query<&item::ItemStorage>,
    q_pos: Query<&Transform>,
    q_it: Query<&IsItem>,
    // asset_server: Res<AssetServer>,
    rpg_folder: Res<RpgTextures>,
) {
    let actor = actors.single();
    if click.just_pressed(MouseButton::Left) || control_settings.check(ControlCode::Use, &keys) {
        let storage = q_st.get(actor).unwrap();
        let item = if let Some(item) = storage.get_index(0) {
            item
        } else {
            return;
        };

        let it = q_it.get(item).unwrap();
        let hold_point =
            q_pos.get(actor).unwrap().translation.truncate() + CHARACTER_LEFT_HAND_OFFSET;
        if it.useable(world, item) {
            it.item_use(&mut commands, world, item, hold_point, coords.0, rpg_folder);
        }
    }
}

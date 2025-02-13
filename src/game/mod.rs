use crate::{assets::RpgTextures, constants::*, control::*};
use bevy::prelude::*;
use item::IsItem;

pub mod health;
pub mod item;
pub mod object;

pub fn inputs_use(
    mut commands: Commands,
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
        if !it.0.check_can_use() {
            return;
        }

        let hold_point =
            q_pos.get(actor).unwrap().translation.truncate() + CHARACTER_LEFT_HAND_OFFSET;

        it.0.item_use(&mut commands, hold_point, coords, rpg_folder);
    }
}

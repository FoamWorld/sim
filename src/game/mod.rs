use crate::{assets::RpgTextures, control::*};
use avian2d::prelude::*;
use bevy::prelude::*;

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
    // asset_server: Res<AssetServer>,
    rpg_folder: Res<RpgTextures>,
) {
    let actor = actors.single();
    if click.just_pressed(MouseButton::Left) || control_settings.check(ControlCode::Use, &keys) {
        let storage = q_st.get(actor).unwrap();
        let _item = if let Some(item) = storage.0[0] {
            item
        } else {
            return;
        };

        let start_point = q_pos.get(actor).unwrap().translation.truncate();
        let ray = coords.0.unwrap() - start_point;
        let unit = ray / ray.length();

        commands.spawn((
            Sprite::from_atlas_image(
                rpg_folder.get_image_handle("spells"),
                rpg_folder.get_texture_atlas("spells", 0),
            ),
            Transform::from_xyz(
                start_point.x + unit.x * 10.0,
                start_point.y + unit.y * 10.0,
                0.0,
            ),
            RigidBody::Dynamic,
            Collider::circle(2.0),
            LockedAxes::ROTATION_LOCKED,
            LinearVelocity(ray * 1.0),
        ));
    }
}

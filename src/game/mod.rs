use crate::control::*;
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
    asset_server: Res<AssetServer>,
) {
    let actor = actors.single();
    if click.just_pressed(MouseButton::Left) || control_settings.check(ControlCode::Use, &keys) {
        let storage = q_st.get(actor).unwrap();
        let _item = if let Some(item) = storage.0[0] {
            item
        } else {
            return;
        };

        let transform = q_pos.get(actor).unwrap();
        let mut linear = coords.0.unwrap() - transform.translation.truncate();
        linear = linear * (200.0 / linear.abs());

        commands.spawn((
            Sprite::from_image(asset_server.get_handle("textures/spells.png").unwrap()),
            *transform,
            RigidBody::Dynamic,
            Collider::circle(2.0),
            LockedAxes::ROTATION_LOCKED,
            LinearVelocity(linear),
        ));
    }
}

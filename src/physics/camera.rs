use crate::{character::IsActive, state::AppState};
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct PrimaryCamera;

pub struct PrimaryCameraPlugin;
impl Plugin for PrimaryCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorCoords>();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn((
                Camera {
                    // hdr: true,
                    clear_color: ClearColorConfig::Custom(Color::srgb(0.0, 0.0, 0.0)),
                    ..default()
                },
                Camera2d,
                PrimaryCamera,
            ));
        });
        app.add_systems(
            Update,
            (translate_cursor_position, rotate_with_mouse).run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Resource, Default)]
pub struct CursorCoords(pub Option<Vec2>);

pub fn translate_cursor_position(
    mut coords: ResMut<CursorCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = if let Ok(window) = q_window.get_single() {
        window
    } else {
        return;
    };
    let ray = window
        .cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)));
    coords.0 = if let Some(Ok(ray3d)) = ray {
        Some(ray3d.origin.truncate())
    } else {
        None
    }
}

#[derive(Component)]
pub struct RotateWithMouse(pub Quat); // offset

pub fn rotate_with_mouse(
    coords: Res<CursorCoords>,
    mut query: Query<(&mut Transform, &RotateWithMouse, &GlobalTransform), With<IsActive>>,
) {
    let dest = if let Some(dest) = coords.0 {
        dest
    } else {
        return;
    };
    for (mut transform, rotate_offset, global_transform) in query.iter_mut() {
        let start = global_transform.translation().truncate();
        let ray = dest - start;
        let rotation = Quat::from_rotation_z(ops::atan2(ray.y, ray.x)).mul_quat(rotate_offset.0);
        transform.rotation = rotation;
    }
}

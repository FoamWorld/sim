use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct PrimaryCamera;

pub struct PrimaryCameraPlugin;
impl Plugin for PrimaryCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorCoords>();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn((Camera2d::default(), PrimaryCamera));
        });
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

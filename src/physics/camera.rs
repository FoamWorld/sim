use super::SceneBox;
use crate::{character::*, constants::*};
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct PrimaryCamera;

#[derive(Resource, Default)]
pub struct CursorCoords(pub Option<Vec2>);

#[derive(Resource, Reflect)]
#[reflect(Resource)]
#[type_path = "sim::physics"]
pub struct CameraMoveConfig {
    /// Stops to move close [`SceneBox`] horizontally.
    pub stop_horizontal_border: bool,

    /// Stops to move close [`SceneBox`] vertically.
    pub stop_vertical_border: bool,

    /// Follows player position.
    pub follow_actor: bool,

    /// Follows cursor position.
    pub follow_cursor: bool,
}

impl Default for CameraMoveConfig {
    fn default() -> Self {
        Self {
            stop_horizontal_border: true,
            stop_vertical_border: true,
            follow_actor: true,
            follow_cursor: false,
        }
    }
}

pub fn update_camera_position(
    scene_box: Res<SceneBox>,
    move_config: Res<CameraMoveConfig>,
    q_actor: Query<&GlobalTransform, With<Actor>>,
    mut commands: Commands,
    q_camera: Query<Entity, With<PrimaryCamera>>,
) {
    let camera = q_camera.single().unwrap();

    let mut vec3 = if move_config.follow_actor {
        q_actor.single().unwrap().translation()
    } else {
        return;
    };

    if move_config.stop_horizontal_border {
        let left_limit = scene_box.horizontal.x + VIEWPORT_WIDTH * 0.5;
        if vec3.x < left_limit {
            vec3.x = left_limit;
        }

        let right_limit = scene_box.horizontal.y - VIEWPORT_WIDTH * 0.5;
        if vec3.x > right_limit {
            vec3.x = right_limit;
        }
    }

    if move_config.stop_vertical_border {
        let low_limit = scene_box.vertical.x + VIEWPORT_HEIGHT * 0.5;
        if vec3.y < low_limit {
            vec3.y = low_limit;
        }

        let high_limit = scene_box.vertical.y - VIEWPORT_HEIGHT * 0.5;
        if vec3.y > high_limit {
            vec3.y = high_limit;
        }
    }

    commands
        .entity(camera)
        .insert(Transform::from_translation(vec3));
}

pub fn update_cursor_position(
    mut coords: ResMut<CursorCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) {
    let (camera, camera_transform) = q_camera.single().unwrap();
    let window = if let Ok(window) = q_window.single() {
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
pub struct RotateWithMouse {
    pub mid: f32,
    pub extent: f32,
}

impl RotateWithMouse {
    pub fn new(tuple: (f32, f32)) -> Self {
        let (mid, extent) = tuple;
        Self { mid, extent }
    }
}

pub fn rotate_with_mouse(
    coords: Res<CursorCoords>,
    status: Res<ActorStatus>,
    mut query: Query<(&mut Transform, &RotateWithMouse, &GlobalTransform), With<IsActive>>,
) {
    let dest = if let Some(dest) = coords.0 {
        dest
    } else {
        return;
    };
    for (mut transform, rotate_config, global_transform) in query.iter_mut() {
        let start = global_transform.translation().truncate();
        let ray = dest - start;
        let theta = ops::atan2(ray.y, ray.x);
        let mid = if status.facing == ActorFacing::Right {
            rotate_config.mid
        } else {
            PI - rotate_config.mid
        };

        let mut extent = theta - mid;
        if extent > PI {
            extent = extent - 2.0 * PI;
        }
        if extent < -PI {
            extent = extent + 2.0 * PI;
        }

        let result = if extent > rotate_config.extent {
            mid + rotate_config.extent
        } else if extent < -rotate_config.extent {
            mid - rotate_config.extent
        } else {
            theta
        };
        let rotation = Quat::from_rotation_z(result);
        transform.rotation = rotation;
    }
}

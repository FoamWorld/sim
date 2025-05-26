use crate::state::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use camera::*;
use collision::*;

pub mod camera;
pub mod collision;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
#[type_path = "sim::physics"]
pub struct SceneBox {
    pub horizontal: Vec2,
    pub vertical: Vec2,
}

pub struct GamePhysicsPlugin;

impl Plugin for GamePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorCoords>()
            .init_resource::<SceneBox>()
            .init_resource::<CameraMoveConfig>();
        app.add_event::<CrashEvent>().add_event::<TouchEvent>();

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

        /*
        app.add_systems(
            PostProcessCollisions,
            inspect_collisions.in_set(InGameSet::Logic),
        );
        */

        app.add_systems(
            Update,
            (
                update_camera_position,
                update_cursor_position,
                rotate_with_mouse,
                read_crash,
            )
                .in_set(InGameSet::Logic),
        );
    }
}

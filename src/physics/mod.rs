use crate::state::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use camera::*;
use collision::*;

pub mod camera;
pub mod collision;

pub struct GamePhysicsPlugin;

impl Plugin for GamePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorCoords>()
            .insert_resource(CameraMoveConfig {
                follow_actor: false,
                with_offset: Vec2::new(0.0, 0.0),
            });
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

        app.add_systems(
            PostProcessCollisions,
            (touch_detection.before(crash_detection), crash_detection)
                .run_if(in_state(AppState::InGame)),
        );

        app.add_systems(
            Update,
            (translate_cursor_position, rotate_with_mouse, read_crash).in_set(InGameSet::Logic),
        );
    }
}

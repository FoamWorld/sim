use crate::state::*;
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
        app.add_event::<TouchEvent>();

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
            (translate_cursor_position, rotate_with_mouse).in_set(InGameSet::Logic),
        );
    }
}

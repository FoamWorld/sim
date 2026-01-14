use crate::state::*;
use bevy::prelude::*;
use camera::*;
use collision::*;
use picking::*;

pub mod camera;
pub mod collision;
pub mod picking;

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
            .init_resource::<CameraMoveConfig>()
            .init_resource::<HoverEntity>();

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
            FixedUpdate,
            apply_tnua_fall_through_controls.in_set(bevy_tnua::TnuaUserControlsSystems),
        );

        app.add_systems(
            FixedUpdate,
            (read_collisions, read_contact_forces)
                .chain()
                .in_set(InGameSet::Logic),
        );

        app.add_systems(
            Update,
            (
                update_cursor_position,
                update_camera_position,
                (rotate_with_mouse, physics_hover_detection),
            )
                .chain()
                .in_set(InGameSet::PostInput),
        );

        let world = app.world_mut();
        world.add_observer(on_touch_start);
        world.add_observer(on_crash);
    }
}

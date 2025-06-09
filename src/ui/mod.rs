use bevy::{
    prelude::*,
    window::{PrimaryWindow, SystemCursorIcon},
    winit::cursor::CursorIcon,
};
use menu::*;

pub mod diagnostics;
pub mod menu;
pub mod message;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(diagnostics::DiagnosticsTextPlugin)
            .add_plugins(message::MessagePlugin);

        app.add_systems(PostStartup, set_cursor);

        app.add_systems(Update, button_system);
    }
}

fn set_cursor(mut commands: Commands, q_window: Query<Entity, With<PrimaryWindow>>) {
    let window = if let Ok(window) = q_window.single() {
        window
    } else {
        return;
    };
    commands
        .entity(window)
        .insert(CursorIcon::System(SystemCursorIcon::Crosshair));
}

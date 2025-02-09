use crate::physics::camera::*;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiagnosticsState {
    Off,
    On,
}

pub struct DiagnosticsTextPlugin;

impl Plugin for DiagnosticsTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.insert_state(DiagnosticsState::Off);
        app.add_systems(Startup, setup_diagnostics_text)
            .add_systems(
                Update,
                |current_state: ResMut<State<DiagnosticsState>>,
                 mut next_state: ResMut<NextState<DiagnosticsState>>,
                 keys: Res<ButtonInput<KeyCode>>| {
                    if keys.just_pressed(KeyCode::F3) {
                        let new_state = match current_state.get() {
                            DiagnosticsState::Off => DiagnosticsState::On,
                            DiagnosticsState::On => DiagnosticsState::Off,
                        };
                        next_state.set(new_state);
                    }
                },
            )
            .add_systems(
                Update,
                (
                    translate_cursor_position,
                    update_diagnostics_text.run_if(in_state(DiagnosticsState::On)),
                ),
            );
    }
}

#[derive(Component)]
struct DiagnosticsText;

fn setup_diagnostics_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new(""),
        TextFont {
            // weight: 300
            font: asset_server.load("fonts/open-sans.regular.ttf"),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::BLACK),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
        DiagnosticsText,
    ));
}

fn update_diagnostics_text(
    diagnostics: Res<DiagnosticsStore>,
    coords: Res<CursorCoords>,
    mut query: Query<&mut Text, With<DiagnosticsText>>,
) {
    for mut text in &mut query {
        let fps_str = if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                format!("{value:.2}")
            } else {
                "Unknown".to_string()
            }
        } else {
            "Broken".to_string()
        };

        let coords_str = if let Some(vec) = coords.0 {
            let x = vec.x;
            let y = vec.y;
            format!("({x:.2}, {y:.2})")
        } else {
            "Outside".to_string()
        };

        text.0 = format!("FPS: {fps_str}\nCursor: {coords_str}");
    }
}

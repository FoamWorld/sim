use bevy::{
    prelude::*,
    window::{PrimaryWindow, SystemCursorIcon},
    winit::cursor::CursorIcon,
};

use crate::state::AppState;

#[derive(Component)]
pub struct WillDestroy;

pub fn set_cursor(mut commands: Commands, q_window: Query<Entity, With<PrimaryWindow>>) {
    let window = if let Ok(window) = q_window.get_single() {
        window
    } else {
        return;
    };
    commands
        .entity(window)
        .insert(CursorIcon::System(SystemCursorIcon::Crosshair));
}

pub fn start_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut background = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
        WillDestroy,
    ));
    background.with_children(|parent: &mut ChildBuilder<'_>| {
        parent.spawn((
            Text::new("Mercury & Milfoil"),
            TextFont {
                // weight:700
                font: asset_server.load("fonts/quattrocento.regular.ttf"),
                font_size: 40.0,
                ..default()
            },
            TextColor(Color::BLACK),
        ));
        parent
            .spawn((
                Button,
                Text::new("Start"),
                TextFont {
                    // weight: 300
                    font: asset_server.load("fonts/open-sans.regular.ttf"),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::BLACK),
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
            ))
            .observe(
                |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<AppState>>| {
                    next_state.set(AppState::InGame);
                },
            );
    });
}

pub fn start_pause(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut background = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.75)),
        WillDestroy,
    ));
    background.with_children(|parent: &mut ChildBuilder<'_>| {
        parent.spawn((
            Text::new("--- paused ---"),
            TextFont {
                // open sans, weight: 300
                font: asset_server.load("fonts/open-sans.regular.ttf"),
                font_size: 40.0,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout {
                justify: JustifyText::Center,
                ..default()
            },
        ));
    });
}

pub fn finish_ui(mut commands: Commands, query: Query<Entity, With<WillDestroy>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

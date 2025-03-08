use crate::{constants::PROJECT_TITLE, scene::*, state::*};
use bevy::{
    prelude::*,
    window::{PrimaryWindow, SystemCursorIcon},
    winit::cursor::CursorIcon,
};

const UI_CLEAR_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const UI_CLOTH_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.75);
const UI_TEXT_COLOR: Color = Color::srgb(0.73, 0.49, 0.17);

#[derive(Component)]
pub struct UiOnce;

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
    with_background(
        &mut commands,
        UI_CLEAR_COLOR,
        |parent: &mut ChildBuilder<'_>| {
            parent.spawn((
                Text::new(PROJECT_TITLE),
                TextFont {
                    font: asset_server.load("fonts/quattrocento.bold.ttf"),
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
            parent
                .spawn((
                    Button,
                    Text::new("Start"),
                    TextFont {
                        font: asset_server.load("fonts/open-sans.regular.ttf"),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(UI_TEXT_COLOR),
                    TextLayout {
                        justify: JustifyText::Center,
                        ..default()
                    },
                ))
                .observe(
                    |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<AppState>>| {
                        next_state.set(AppState::ModeSelection);
                    },
                );
        },
    );
}

pub fn start_mode_selection(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mode_list = vec![
        "Gallery",
        #[cfg(feature = "devtools")]
        "Sandbox",
    ];

    let font = TextFont {
        font: asset_server.load("fonts/open-sans.regular.ttf"),
        font_size: 16.0,
        ..default()
    };

    with_background(
        &mut commands,
        UI_CLEAR_COLOR,
        |parent: &mut ChildBuilder<'_>| {
            for mode in mode_list {
                add_button(
                    parent,
                    mode,
                    font.clone(),
                    |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<AppState>>| {
                        next_state.set(AppState::InGame);
                    },
                );
            }
        },
    );
}

pub fn start_pause(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = TextFont {
        font: asset_server.load("fonts/open-sans.regular.ttf"),
        font_size: 16.0,
        ..default()
    };

    with_background(&mut commands, UI_CLOTH_COLOR, |parent| {
        add_button(
            parent,
            "resume",
            font.clone(),
            |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                next_state.set(GameState::Running);
            },
        );
        add_button(
            parent,
            "save",
            font.clone(),
            |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<ProcessState>>| {
                next_state.set(ProcessState::PreSaveScene);
            },
        );
    });
}

pub fn finish_ui(mut commands: Commands, query: Query<Entity, With<UiOnce>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn with_background(commands: &mut Commands, color: Color, f: impl FnOnce(&mut ChildBuilder<'_>)) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(color),
            UiOnce,
        ))
        .with_children(f);
}

fn add_button<E: Event, B: Bundle, M>(
    parent: &mut ChildBuilder,
    name: &str,
    font: TextFont,
    observer: impl bevy::ecs::system::IntoObserverSystem<E, B, M>,
) {
    parent
        .spawn((
            Button,
            Text::new(name),
            font,
            TextColor(UI_TEXT_COLOR),
            TextLayout {
                justify: JustifyText::Center,
                ..default()
            },
        ))
        .observe(observer);
}

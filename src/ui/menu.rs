use crate::{assets::*, constants::*, markers::UiRoot, scene::*, state::*};
use bevy::prelude::*;

const UI_CLEAR_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const UI_CLOTH_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.75);
const UI_TEXT_COLOR: Color = Color::srgb(0.73, 0.49, 0.17);

#[derive(Component)]
pub enum ButtonType {
    Start,
    Quit,
    ToGame(String),
    Resume,
    Save,
    BackToMenu,
}

pub fn button_system(
    query: Query<(&Interaction, &ButtonType), Changed<Interaction>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_process_state: ResMut<NextState<ProcessState>>,
    mut target: ResMut<PortalTarget>,
    mut writer_exit: MessageWriter<AppExit>,
) {
    for (interaction, btn_type) in &query {
        if *interaction == Interaction::Pressed {
            match btn_type {
                ButtonType::Start => next_app_state.set(AppState::ModeSelection),
                ButtonType::Quit => {
                    writer_exit.write(AppExit::Success);
                }
                ButtonType::ToGame(string) => {
                    target.set_if_neq(PortalTarget::Unique(string.to_string()));
                    next_app_state.set(AppState::InGame);
                }
                ButtonType::Resume => next_game_state.set(GameState::Running),
                ButtonType::Save => next_process_state.set(ProcessState::PreSaveScene),
                ButtonType::BackToMenu => next_app_state.set(AppState::Menu),
            }
        }
    }
}

pub fn start_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = TextFont {
        font: asset_server.load("fonts/open-sans.regular.ttf"),
        font_size: 17.3,
        ..default()
    };

    with_background(
        &mut commands,
        UI_CLEAR_COLOR,
        |parent: &mut ChildSpawnerCommands| {
            parent.spawn((
                Text::new(PROJECT_TITLE),
                TextFont {
                    font: asset_server.load("fonts/quattrocento.bold.ttf"),
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::BLACK),
                Node {
                    padding: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new(PROJECT_SUBTITLE),
                TextFont {
                    font: asset_server.load("fonts/open-sans.light-italic.ttf"),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb_u8(0xb0, 0xb0, 0xb0)),
                Node {
                    padding: UiRect::bottom(Val::Px(48.0)),
                    ..default()
                },
            ));
            add_button(parent, "Start", font.clone(), ButtonType::Start);
            add_button(parent, "Quit", font.clone(), ButtonType::Quit);
        },
    );
}

pub fn start_mode_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    modes: Res<Assets<ModesConfig>>,
    modes_id: Res<ModesConfigHandle>,
) {
    let table = modes.get(modes_id.0.id()).unwrap();
    #[cfg(not(feature = "devtools"))]
    let level = table.level;
    #[cfg(feature = "devtools")]
    let level = 0xffffu16;

    let font = TextFont {
        font: asset_server.load("fonts/open-sans.regular.ttf"),
        font_size: 16.0,
        ..default()
    };

    with_background(
        &mut commands,
        UI_CLEAR_COLOR,
        |parent: &mut ChildSpawnerCommands| {
            for mode in &table.modes {
                if mode.level > level {
                    continue;
                }
                add_button(
                    parent,
                    &mode.title,
                    font.clone(),
                    ButtonType::ToGame(mode.entrance.clone()),
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
        add_button(parent, "resume", font.clone(), ButtonType::Resume);
        add_button(parent, "save", font.clone(), ButtonType::Save);
        add_back_to_menu_button(parent, font);
    });
}

pub fn finish_ui(mut commands: Commands, query: Query<Entity, With<UiRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn with_background(
    commands: &mut Commands,
    color: Color,
    f: impl FnOnce(&mut ChildSpawnerCommands),
) {
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
            ZIndex(1),
            UiRoot::Menu,
        ))
        .with_children(f);
}

fn add_button(parent: &mut ChildSpawnerCommands, name: &str, font: TextFont, but_type: ButtonType) {
    parent.spawn((
        Button,
        but_type,
        Text::new(name),
        font,
        TextColor(UI_TEXT_COLOR),
        TextLayout {
            justify: Justify::Center,
            ..default()
        },
    ));
}

fn add_back_to_menu_button(parent: &mut ChildSpawnerCommands, font: TextFont) {
    add_button(parent, "back to menu", font, ButtonType::BackToMenu);
}

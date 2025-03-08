use crate::{assets::*, control::*, ui::*};
use avian2d::prelude::*;
use bevy::{asset::*, prelude::*};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Initialize,
    Menu,
    ModeSelection,
    InGame,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    /// locked because it's not ready yet
    Locked,
    Running,
    Pause,
    Processing,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InGameSet {
    /// user input
    Input,
    /// treat input
    PostInput,
    // game logic
    Logic,
    // ui update
    Ui,
}

#[derive(Component)]
pub struct WontRemove;

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        // Configuration
        app.insert_state(AppState::Initialize)
            .insert_state(GameState::Locked);

        app.configure_sets(
            Update,
            (
                InGameSet::PostInput.after(InGameSet::Input),
                InGameSet::Input,
                InGameSet::Logic,
                InGameSet::Ui,
            )
                .run_if(in_state(GameState::Running))
                .run_if(in_state(AppState::InGame)),
        );

        // AppState::Initialize
        app.add_systems(PostStartup, (load_textures, set_cursor))
            .add_systems(
                Update,
                check_textures.run_if(in_state(AppState::Initialize)),
            );

        // AppState::Menu
        app.add_systems(OnEnter(AppState::Menu), start_menu)
            .add_systems(OnExit(AppState::Menu), finish_ui);

        // AppState::ModeSelection
        app.add_systems(OnEnter(AppState::ModeSelection), start_mode_selection)
            .add_systems(OnExit(AppState::ModeSelection), finish_ui);

        // AppState::InGame
        app.add_systems(
            OnExit(AppState::InGame),
            (
                remove_all,
                |mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::Locked);
                },
            )
                .chain(),
        )
        .add_systems(Update, toggle_pause.in_set(InGameSet::Input))
        .add_systems(OnEnter(GameState::Running), exit_pause)
        .add_systems(OnExit(GameState::Running), enter_pause);
    }
}

fn toggle_pause(
    mut next_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    control_settings: Res<ControlSettings>,
) {
    if control_settings.check(ControlCode::Pause, &keys) {
        next_state.set(GameState::Pause);
    }
}

fn enter_pause(
    commands: Commands,
    mut time: ResMut<Time<Physics>>,
    asset_server: Res<AssetServer>,
) {
    time.pause();
    start_pause(commands, asset_server);
}

fn exit_pause(
    commands: Commands,
    mut time: ResMut<Time<Physics>>,
    query: Query<Entity, With<UiOnce>>,
) {
    time.unpause();
    finish_ui(commands, query);
}

fn remove_all(
    world: &mut World,
    query: &mut QueryState<Entity, (Without<WontRemove>, Without<Window>)>,
) {
    let mut vec: Vec<Entity> = vec![];
    for entity in query.iter_mut(world) {
        vec.push(entity);
    }
    for entity in vec {
        world.despawn(entity);
    }
}

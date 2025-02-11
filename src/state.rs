use crate::{
    assets::*,
    character::*,
    control::*,
    physics::{collision::*, room::*},
    ui::*,
};
use avian2d::prelude::*;
use bevy::{asset::*, prelude::*};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    Menu,
    InGame,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunState {
    Running,
    Paused,
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(AppState::Loading)
            .insert_state(RunState::Running);

        // while loading
        app.add_systems(OnEnter(AppState::Loading), load_textures)
            .add_systems(Update, check_textures.run_if(in_state(AppState::Loading)));

        // while at menu
        app.add_systems(OnEnter(AppState::Menu), start_menu)
            .add_systems(OnExit(AppState::Menu), finish_ui);

        // while in game
        app.add_event::<CrashEvent>();
        app.add_systems(
            OnEnter(AppState::InGame),
            (setup_game, set_cursor, setup_character),
        );

        app.add_systems(
            PostProcessCollisions,
            touch_detection.before(crash_detection).run_if(in_state(AppState::InGame)),
        );

        app.add_systems(OnEnter(RunState::Paused), enter_pause)
            .add_systems(OnExit(RunState::Paused), exit_pause)
            .add_systems(Update, toggle_pause.run_if(in_state(AppState::InGame)));
    }
}

fn toggle_pause(
    current_state: ResMut<State<RunState>>,
    mut next_state: ResMut<NextState<RunState>>,
    keys: Res<ButtonInput<KeyCode>>,
    control_settings: Res<ControlSettings>,
) {
    if control_settings.check(ControlCode::Pause, &keys) {
        let new_state = match current_state.get() {
            RunState::Paused => RunState::Running,
            RunState::Running => RunState::Paused,
        };
        next_state.set(new_state);
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
    query: Query<Entity, With<WillDestroy>>,
) {
    time.unpause();
    finish_ui(commands, query);
}

fn setup_game(mut commands: Commands, rpg_folder: Res<RpgTextures>) {
    spawn_room(&mut commands);

    let sprite = Sprite::from_atlas_image(
        rpg_folder.get_image_handle("sign"),
        rpg_folder.get_texture_atlas("sign", 1),
    );
    spawn_sign(&mut commands, -200.0, 0.0, sprite);
}

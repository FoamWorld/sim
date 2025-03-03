use crate::{assets::*, character::*, control::*, game::mob::health::*, physics::collision::*, ui::*};
use avian2d::prelude::*;
use bevy::{asset::*, prelude::*};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Initialize,
    Menu,
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
        app.add_systems(Startup, (load_textures, set_cursor))
            .add_systems(
                Update,
                check_textures.run_if(in_state(AppState::Initialize)),
            );

        // AppState::Menu
        app.add_systems(OnEnter(AppState::Menu), start_menu)
            .add_systems(OnExit(AppState::Menu), finish_ui);

        // AppState::InGame
        app.init_resource::<SelectedSlot>();
        app.add_systems(Update, toggle_pause.in_set(InGameSet::Input));

        /* app.add_systems(
            FixedUpdate,
            (
                // add game logic here
            ).in_set(InGameSet::Logic),
        ); */

        app.add_systems(
            PostProcessCollisions,
            (touch_detection.before(crash_detection), crash_detection)
                .run_if(in_state(AppState::InGame)),
        );

        app.add_event::<CrashEvent>()
            .add_event::<HealthClearedEvent>();
        app.add_systems(OnEnter(GameState::Running), exit_pause)
            .add_systems(OnExit(GameState::Running), enter_pause)
            .add_systems(
                Update,
                (read_crash, read_health_cleared).run_if(in_state(AppState::InGame)),
            );
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
    query: Query<Entity, With<WillDestroy>>,
) {
    time.unpause();
    finish_ui(commands, query);
}

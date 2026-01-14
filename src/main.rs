#![allow(dead_code, reason = "developing period")]

extern crate bevy;
extern crate bevy_common_assets;
extern crate bevy_rapier2d;
extern crate bevy_tnua;
extern crate bevy_tnua_rapier2d;
extern crate serde;

use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowResolution},
};
use bevy_common_assets::toml;
use bevy_rapier2d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_rapier2d::*;

mod assets;
mod character;
mod control;
mod game;
mod markers;
mod physics;
mod scene;
mod state;
mod statistics;
mod ui;

mod constants;
use constants::*;

#[bevy_main]
fn main() {
    let mut app = App::new();
    let window = Window {
        title: PROJECT_TITLE.into(),
        resolution: WindowResolution::new(1000.0, 750.0).with_scale_factor_override(SCALE_FACTOR),
        enabled_buttons: EnabledButtons {
            minimize: true,
            maximize: false,
            close: true,
        },
        resizable: false,
        decorations: true,
        ..default()
    };

    // Core plugins.
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }),
        state::AppStatePlugin,
    ));

    // Game necessity.
    app.add_plugins((
        control::ControlPlugin,
        game::GamePlugin,
        scene::RegisteryPlugin,
        toml::TomlAssetPlugin::<assets::ModesConfig>::new(&["modes.toml"]),
    ));

    // Physics.
    app.add_plugins((
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER),
        TnuaControllerPlugin::new(FixedUpdate),
        TnuaRapier2dPlugin::new(FixedUpdate),
        physics::GamePhysicsPlugin,
    ));

    // Misc.
    app.add_plugins((
        ui::UiPlugin,
        statistics::StatisticsPlugin,
    ));

    // Run.
    app.add_systems(Startup, assets::setup).run();
}

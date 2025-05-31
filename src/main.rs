#![allow(dead_code, reason = "developing period")]

extern crate bevy;
extern crate bevy_rapier2d;
extern crate serde;

use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowResolution},
};
use bevy_rapier2d::prelude::*;

mod assets;
mod character;
mod control;
mod diagnostics;
mod game;
mod message;
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
        resolution: WindowResolution::new(1000.0, 750.0).with_scale_factor_override(2.0),
        enabled_buttons: EnabledButtons {
            minimize: true,
            maximize: false,
            close: true,
        },
        resizable: false,
        decorations: true,
        ..default()
    };
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }),
        RapierPhysicsPlugin::<physics::collision::MyPhysicsHooks>::pixels_per_meter(UNIT_PER_METER),
        state::AppStatePlugin,
        diagnostics::DiagnosticsTextPlugin,
        control::ControlPlugin,
        physics::GamePhysicsPlugin,
        message::MessagePlugin,
        scene::RegisteryPlugin,
        statistics::StatisticsPlugin,
        game::GamePlugin,
    ));
    app.run();
}

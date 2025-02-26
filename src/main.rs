extern crate avian2d;
extern crate bevy;
extern crate serde;

use avian2d::prelude::*;
use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowResolution},
};

mod assets;
mod character;
mod control;
mod diagnostics;
mod game;
mod message;
mod physics;
mod scene;
mod state;
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
        PhysicsPlugins::default().with_length_unit(UNIT_PER_METER),
        state::AppStatePlugin,
        diagnostics::DiagnosticsTextPlugin,
        control::ControlPlugin,
        physics::camera::PrimaryCameraPlugin,
        message::MessagePlugin,
        scene::RegisteryPlugin,
    ));
    app.insert_resource(Gravity(Vec2::NEG_Y * 628.0));
    app.run();
}

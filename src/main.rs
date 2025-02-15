extern crate avian2d;
extern crate bevy;

use avian2d::{math::*, prelude::*};
use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowResolution},
};

mod assets;
mod character;
mod game;
mod scene;
mod ui;

mod message;
use message::*;

mod control;
use control::*;

mod constants;
use constants::*;

mod physics;
use physics::camera::*;

mod state;
use state::*;

mod diagnostics;
use diagnostics::*;

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
        AppStatePlugin,
        DiagnosticsTextPlugin,
        ControlPlugin,
        PrimaryCameraPlugin,
        MessagePlugin,
        game::RegisteryPlugin,
    ));
    app.insert_resource(Gravity(Vector::ZERO));
    app.run();
}

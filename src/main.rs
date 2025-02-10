extern crate avian2d;
extern crate bevy;

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

mod ui;

mod assets;

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

mod game;

#[bevy_main]
fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "sim".into(),
                resolution: (VIEWPORT_WIDTH, VIEWPORT_HEIGHT).into(),
                resizable: false,
                decorations: true,
                ..default()
            }),
            ..default()
        }),
        PhysicsPlugins::default().with_length_unit(UNIT_PER_METER),
        AppStatePlugin,
        DiagnosticsTextPlugin,
        ControlPlugin,
        PrimaryCameraPlugin,
        MessagePlugin,
    ));
    app.insert_resource(Gravity(Vector::ZERO));
    app.run();
}

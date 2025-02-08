extern crate avian2d;
extern crate bevy;

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

mod ui;
use ui::*;

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

#[bevy_main]
fn main() {
    let mut app = App::new();
    // app.add_resource(Msaa { samples: 4 })
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "sim".into(),
                resolution: (800.0, 608.0).into(),
                resizable: false,
                decorations: true,
                ..default()
            }),
            ..default()
        }),
        PhysicsPlugins::default().with_length_unit(UNIT_PER_METER),
        MessagePlugin,
        AppStatePlugin,
    ));
    app.insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)));
    app.insert_resource(Gravity(Vector::ZERO));
    app.add_systems(Startup, setup)
        .add_systems(Update, (inputs_move, inputs_wait));
    app.add_systems(Startup, start_menu);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d::default(), PrimaryCamera));
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.8, 0.1),
            custom_size: Some(Vec2::splat(16.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        RigidBody::Dynamic,
        Collider::circle(8.0),
        LockedAxes::ROTATION_LOCKED,
        MovementSpeed(100.0),
        Actor,
    ));
}

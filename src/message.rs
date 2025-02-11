use crate::{physics::collision::*, state::AppState};
use bevy::prelude::*;

pub struct MessagePlugin;

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TouchEvent>();
        app.add_systems(Startup, init_info_text);
        app.add_systems(Update, read_touch_sign.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct InfoText;

fn init_info_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new(""),
        TextFont {
            font: asset_server.load("fonts/open-sans.regular.ttf"),
            font_size: 20.0,
            ..default()
        },
        TextColor(bevy::color::palettes::css::DARK_GRAY.into()),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        InfoText,
    ));
}

#[derive(Component)]
pub struct Sign(pub String);

// todo: add duration
fn read_touch_sign(
    mut reader: EventReader<TouchEvent>,
    mut query_text: Query<&mut Text, With<InfoText>>,
    query_sign: Query<&Sign>,
) {
    for ev in reader.read() {
        let result = query_sign.get(ev.0);
        if result.is_ok() {
            let sign = result.unwrap();
            for mut text in &mut query_text {
                text.0 = sign.0.clone();
            }
        }
    }
}

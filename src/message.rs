use crate::{physics::collision::*, state::*};
use bevy::prelude::*;

#[derive(Event)]
pub struct MessageEvent {
    message: String,
}

impl MessageEvent {
    pub fn info(str: &str) -> Self {
        Self {
            message: str.to_string(),
        }
    }
}

pub struct MessagePlugin;

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MessageEvent>();
        app.add_systems(FixedUpdate, read_touch_sign.in_set(InGameSet::Logic));
        app.add_systems(Update, read_message_event.in_set(InGameSet::Ui));
    }
}

#[derive(Component)]
pub struct InfoText;

// todo: add duration
fn read_message_event(
    mut reader: EventReader<MessageEvent>,
    mut query_text: Query<&mut Text, With<InfoText>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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
    for ev in reader.read() {
        let mut text = query_text.single_mut();
        // todo: move
        text.0 = ev.message.clone();
    }
}

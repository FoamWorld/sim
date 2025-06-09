use crate::{markers::*, physics::collision::*, state::*};
use bevy::prelude::*;
use std::collections::VecDeque;

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

#[derive(Resource)]
pub struct MessageQueue {
    limit: usize,
    queue_message: VecDeque<String>,
    // todo: (awards, fate) queue_metainfo
}

impl MessageQueue {
    fn new(limit: usize) -> Self {
        Self {
            limit,
            queue_message: VecDeque::<String>::new(),
        }
    }

    fn push(&mut self, str: String) {
        if self.queue_message.len() < self.limit {
            self.queue_message.push_back(str);
        }
    }

    fn pop(&mut self) -> Option<String> {
        self.queue_message.pop_front()
    }
}

pub struct MessagePlugin;

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MessageQueue::new(20));
        app.add_event::<MessageEvent>();

        app.add_systems(
            FixedUpdate,
            (
                process_despawn_timeout,
                read_touch_sign,
                read_message_event,
                message_timer,
            )
                .in_set(InGameSet::Logic),
        );
    }
}

fn message_timer(
    asset_server: Res<AssetServer>,
    mut queue: ResMut<MessageQueue>,
    mut commands: Commands,
) {
    if let Some(str) = queue.pop() {
        commands.spawn((
            Text::new(str.as_str()),
            TextFont {
                font: asset_server.load("fonts/open-sans.regular.ttf"),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()
            },
            Outline::new(Val::Px(1.0), Val::ZERO, Color::WHITE),
            DespawnTimeout(Timer::from_seconds(1.0, TimerMode::Once)),
            UiRoot::GameTemporary,
        ));
    }
}

fn read_message_event(mut reader: EventReader<MessageEvent>, mut queue: ResMut<MessageQueue>) {
    for ev in reader.read() {
        queue.push(ev.message.clone());
    }
}

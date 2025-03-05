use crate::{physics::collision::*, state::*};
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
        app.add_systems(FixedUpdate, read_touch_sign.in_set(InGameSet::Logic));
        app.add_systems(
            Update,
            (
                message_timer.in_set(InGameSet::Logic),
                read_message_event.in_set(InGameSet::Ui),
            ),
        );
    }
}

#[derive(Component)]
pub struct MessageText(pub Timer);

fn message_timer(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut queue: ResMut<MessageQueue>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut MessageText)>,
) {
    if let Ok((entity, mut text)) = query.get_single_mut() {
        text.0.tick(time.delta());

        if text.0.finished() {
            commands.entity(entity).despawn();
        } else {
            return;
        }
    }
    if let Some(str) = queue.pop() {
        commands.spawn((
            Text::new(str.as_str()),
            TextFont {
                font: asset_server.load("fonts/open-sans.regular.ttf"),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::linear_rgb(0.8, 0.8, 0.8)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()
            },
            Outline::new(Val::Px(1.0), Val::ZERO, Color::WHITE),
            MessageText(Timer::from_seconds(1.0, TimerMode::Once)),
        ));
    }
}

fn read_message_event(mut reader: EventReader<MessageEvent>, mut queue: ResMut<MessageQueue>) {
    for ev in reader.read() {
        queue.push(ev.message.clone());
    }
}

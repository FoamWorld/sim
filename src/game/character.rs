use crate::character::*;
use bevy::prelude::*;

pub fn update_actor_position(
    actors: Query<(&Transform, &Actor)>,
    mut position: ResMut<ActorPosition>,
) {
    if actors.is_empty() {
        return;
    }

    let (transform, actor) = actors.single();
    position.center = transform.translation.truncate();
    position.primary_hand_offset = actor.get_primary_hand_offset();
}

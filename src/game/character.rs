use crate::character::*;
use bevy::prelude::*;

pub fn update_actor_position(
    actors: Query<(&Transform, &Actor)>,
    mut position: ResMut<ActorPosition>,
) {
    if let Ok((transform, actor)) = actors.single() {
        position.facing_right = actor.0 == ActorFacing::Right;
        position.center = transform.translation.truncate();
        position.facing_offset = actor.get_facing_offset();
        position.primary_hand_offset = actor.get_primary_hand_offset();
    }
}

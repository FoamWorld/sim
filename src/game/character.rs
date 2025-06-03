use crate::character::*;
use bevy::prelude::*;

pub fn update_actor_status(actors: Query<(&Transform, &Actor)>, mut status: ResMut<ActorStatus>) {
    if let Ok((transform, actor)) = actors.single() {
        let facing = actor.0;
        status.facing = facing;
        status.center = transform.translation.truncate();
        status.facing_offset = facing.get_facing_offset();
        status.primary_hand_offset = facing.get_primary_hand_offset();
    }
}

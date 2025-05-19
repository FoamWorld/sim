use super::inventory::*;
use crate::character::*;
use bevy::prelude::*;

pub fn update_actor_position(
    actors: Query<(Entity, &Transform, &Actor)>,
    changed: Query<Entity, Changed<Actor>>,
    mut position: ResMut<ActorPosition>,
    mut writer: EventWriter<InventorySelectedUpdateEvent>,
) {
    if let Ok((entity, transform, actor)) = actors.single() {
        position.center = transform.translation.truncate();
        position.facing_offset = actor.get_facing_offset();
        position.primary_hand_offset = actor.get_primary_hand_offset();

        if changed.contains(entity) {
            writer.write(InventorySelectedUpdateEvent);
        }
    }
}

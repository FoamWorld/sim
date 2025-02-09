use crate::control::Actor;
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct TouchEvent(pub Entity);

pub fn collision_detection(
    mut query_player: Query<Entity, (With<Actor>, With<RigidBody>)>,
    mut query_pillow: Query<Entity, (With<RigidBody>, With<RigidBodyDisabled>)>,
    mut collisions: ResMut<Collisions>,
    mut writer: EventWriter<TouchEvent>,
) {
    collisions.retain(|contacts| {
        let (pillow, other_entity) = if let Ok(pillow) = query_pillow.get_mut(contacts.entity1) {
            (pillow, contacts.entity2)
        } else if let Ok(pillow) = query_pillow.get_mut(contacts.entity2) {
            (pillow, contacts.entity1)
        } else {
            return true;
        };
        if Ok(other_entity) != query_player.get_mut(other_entity) {
            return true;
        }
        writer.send(TouchEvent(pillow));
        false
    });
}

use crate::{
    character::Actor,
    game::health::{Health, HealthClearedEvent},
};
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct CrashEvent(pub Entity, pub Entity);

pub fn crash_detection(
    query_sufferer: Query<Entity, With<Health>>,
    mut collisions: ResMut<Collisions>,
    mut writer: EventWriter<CrashEvent>,
) {
    collisions.retain(|contacts| {
        if query_sufferer.contains(contacts.entity1) {
            writer.send(CrashEvent(contacts.entity1, contacts.entity2));
        } else if query_sufferer.contains(contacts.entity2) {
            writer.send(CrashEvent(contacts.entity2, contacts.entity1));
        }
        true
    });
}

pub fn read_crash(
    mut reader: EventReader<CrashEvent>,
    mut writer: EventWriter<HealthClearedEvent>,
    mut q_h: Query<&mut Health>,
) {
    for crash in reader.read() {
        let sufferer = crash.0;
        let mut health = q_h.get_mut(sufferer).unwrap();
        health.shift(-1.0);
        if !health.is_alive() {
            writer.send(HealthClearedEvent(sufferer));
        }
    }
}

#[derive(Event)]
pub struct TouchEvent(pub Entity);

pub fn touch_detection(
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

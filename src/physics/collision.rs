use crate::{
    character::Actor,
    game::mob::health::{Health, HealthClearedEvent},
    message::MessageEvent,
};
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct BackgroundLevel;

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
        if let Ok(mut health) = q_h.get_mut(sufferer) {
            health.shift(-1.0);
            if !health.is_alive() {
                writer.send(HealthClearedEvent(sufferer));
            }
        }
    }
}

#[derive(Event)]
pub struct TouchEvent(pub Entity);

pub fn touch_detection(
    mut query_player: Query<Entity, (With<Actor>, With<RigidBody>)>,
    mut query_pillow: Query<Entity, (With<RigidBody>, With<BackgroundLevel>)>,
    mut collisions: ResMut<Collisions>,
    mut writer: EventWriter<TouchEvent>,
) {
    collisions.retain(|contacts| {
        // Already collided.
        let any_penetrating = contacts.manifolds.iter().any(|manifold| {
            manifold
                .contacts
                .iter()
                .any(|contact| contact.penetration > 0.0)
        });
        if any_penetrating {
            return true;
        }

        // Check touch.
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

#[derive(Reflect, Component)]
#[reflect(Component)]
#[type_path = "sim::utils"]
pub struct Sign(pub String);

pub fn read_touch_sign(
    mut reader: EventReader<TouchEvent>,
    mut writer: EventWriter<MessageEvent>,
    query_sign: Query<&Sign>,
) {
    for ev in reader.read() {
        let result = query_sign.get(ev.0);
        if result.is_ok() {
            let sign = result.unwrap();
            writer.send(MessageEvent::info(sign.0.as_str()));
        }
    }
}

pub struct Portal;

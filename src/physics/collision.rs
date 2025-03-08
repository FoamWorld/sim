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

#[derive(Event)]
pub struct TouchEvent(pub Entity);

pub fn inspect_collisions(
    query_player: Query<&Actor, With<RigidBody>>,
    query_background: Query<&BackgroundLevel, With<RigidBody>>,
    query_health: Query<&Health>,
    mut collisions: ResMut<Collisions>,
    mut writer_touch: EventWriter<TouchEvent>,
    mut writer_crash: EventWriter<CrashEvent>,
) {
    collisions.retain(|contacts| {
        let e1 = contacts.entity1;
        let e2 = contacts.entity2;
        let bg1 = query_background.contains(e1);
        let bg2 = query_background.contains(e2);

        if bg1 && bg2 {
            return false;
        }

        // Already collided.
        let any_penetrating = contacts.manifolds.iter().any(|manifold| {
            manifold
                .contacts
                .iter()
                .any(|contact| contact.penetration > 0.0)
        });
        if any_penetrating {
            return !bg1 && !bg2;
        }

        // Check crash.
        if query_health.contains(e1) && !bg2 {
            writer_crash.send(CrashEvent(e1, e2));
        }
        if query_health.contains(e2) && !bg1 {
            writer_crash.send(CrashEvent(e2, e1));
        }

        // Check touch.
        let (pillow, other_entity) = if bg1 {
            (e1, e2)
        } else if bg2 {
            (e2, e1)
        } else {
            return true;
        };
        if !query_player.contains(other_entity) {
            return true;
        }
        writer_touch.send(TouchEvent(pillow));
        false
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

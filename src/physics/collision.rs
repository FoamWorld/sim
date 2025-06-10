use crate::{
    control::MoveDownTimer,
    game::mob::{
        dummy::DamageSensor,
        health::{Health, HealthClearedEvent},
    },
    ui::message::MessageEvent,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::*;

pub fn apply_tnua_fall_through_controls(
    mut query: Query<(&mut TnuaProximitySensor, &TnuaGhostSensor, &MoveDownTimer)>,
) {
    for (mut proximity_sensor, ghost_sensor, timer) in query.iter_mut() {
        if timer.0.finished() {
            for ghost_platform in ghost_sensor.iter() {
                if 1.0 <= ghost_platform.proximity {
                    proximity_sensor.output = Some(ghost_platform.clone());
                    break;
                }
            }
        }
    }
}

#[derive(Event)]
pub struct TouchStartedEvent {
    pub object: Entity,
}

#[derive(Event)]
pub struct CrashEvent {
    pub object: Entity,
    pub force: f32,
}

pub fn write_crash(
    query_health: Query<&Health>,
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut write_touch: EventWriter<TouchStartedEvent>,
    mut writer_crash: EventWriter<CrashEvent>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(h1, _, _) = event {
            write_touch.write(TouchStartedEvent { object: *h1 });
        }
    }

    for event in contact_force_events.read() {
        if query_health.contains(event.collider1) {
            writer_crash.write(CrashEvent {
                object: event.collider1,
                force: event.total_force.length(),
            });
        }

        if query_health.contains(event.collider2) {
            writer_crash.write(CrashEvent {
                object: event.collider2,
                force: event.total_force.length(),
            });
        }
    }
}

pub fn read_crash(
    mut reader: EventReader<CrashEvent>,
    mut writer: EventWriter<HealthClearedEvent>,
    mut writer_event: EventWriter<MessageEvent>,
    sensing: Query<&DamageSensor>,
    mut q_h: Query<&mut Health>,
) {
    for crash in reader.read() {
        let sufferer = crash.object;
        if sensing.contains(sufferer) {
            writer_event.write(MessageEvent::info(
                format!("Damage: {0:.2}", crash.force * 1e-5).as_str(),
            ));
        }
        if let Ok(mut health) = q_h.get_mut(sufferer) {
            health.shift(-crash.force);
            if !health.is_alive() {
                writer.write(HealthClearedEvent(sufferer));
            }
        }
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
#[type_path = "sim::sensor"]
pub struct Sign(pub String);

pub fn read_touch_sign(
    mut reader: EventReader<TouchStartedEvent>,
    mut writer: EventWriter<MessageEvent>,
    query_sign: Query<&Sign>,
) {
    for ev in reader.read() {
        let result = query_sign.get(ev.object);
        if result.is_ok() {
            let sign = result.unwrap();
            writer.write(MessageEvent::info(sign.0.as_str()));
        }
    }
}

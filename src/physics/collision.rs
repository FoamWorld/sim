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
        if timer.0.is_finished() {
            for ghost_platform in ghost_sensor.iter() {
                if 1.0 <= ghost_platform.proximity {
                    proximity_sensor.output = Some(ghost_platform.clone());
                    break;
                }
            }
        }
    }
}

#[derive(EntityEvent)]
pub struct TouchStartedEvent(pub Entity);

#[derive(EntityEvent)]
pub struct CrashEvent {
    pub entity: Entity,
    pub force: f32,
}

pub fn read_collisions(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(h1, _, _) = event {
            commands.trigger(TouchStartedEvent(*h1));
        }
    }
}

pub fn read_contact_forces(
    mut commands: Commands,
    query_health: Query<&Health>,
    mut contact_force_events: MessageReader<ContactForceEvent>,
) {
    for event in contact_force_events.read() {
        if query_health.contains(event.collider1) {
            commands.trigger(CrashEvent {
                entity: event.collider1,
                force: event.total_force.length(),
            });
        }

        if query_health.contains(event.collider2) {
            commands.trigger(CrashEvent {
                entity: event.collider2,
                force: event.total_force.length(),
            });
        }
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
#[type_path = "sim::sensor"]
pub struct Sign(pub String);

pub fn on_touch_start(
    touch_start: On<TouchStartedEvent>,
    query_sign: Query<&Sign>,
    mut writer: MessageWriter<MessageEvent>,
) {
    if let Ok(sign) = query_sign.get(touch_start.0) {
        writer.write(MessageEvent::info(sign.0.as_str()));
    }
}

pub fn on_crash(
    crash: On<CrashEvent>,
    mut commands: Commands,
    mut query_health: Query<&mut Health>,
    sensing: Query<&DamageSensor>,
    mut writer: MessageWriter<MessageEvent>,
) {
    let sufferer = crash.entity;

    // TODO: add health change sensing as middle layer
    if sensing.contains(sufferer) {
        writer.write(MessageEvent::info(
            format!("Damage: {0:.2}", crash.force * 1e-5).as_str(),
        ));
    }

    if let Ok(mut health) = query_health.get_mut(sufferer) {
        health.shift(-crash.force);
        if !health.is_alive() {
            commands.trigger(HealthClearedEvent(sufferer));
        }
    }
}

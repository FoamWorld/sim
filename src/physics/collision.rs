use crate::{
    character::Character,
    game::mob::health::{Health, HealthClearedEvent},
    message::MessageEvent,
};
use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct BackgroundLevel;

#[derive(Component)]
pub struct OneWayPlatform;

#[derive(SystemParam)]
pub struct MyPhysicsHooks<'w, 's> {
    platforms: Query<'w, 's, &'static OneWayPlatform>,
    users: Query<'w, 's, &'static Character>,
    velocities: Query<'w, 's, &'static Velocity>,
}

impl BevyPhysicsHooks for MyPhysicsHooks<'_, '_> {
    fn filter_contact_pair(&self, context: PairFilterContextView) -> Option<SolverFlags> {
        let entity1 = context.collider1();
        let entity2 = context.collider2();

        let (platform_entity, user_entity) =
            if self.platforms.contains(entity1) && self.users.contains(entity2) {
                (entity1, entity2)
            } else if self.platforms.contains(entity2) && self.users.contains(entity1) {
                (entity2, entity1)
            } else {
                return Some(SolverFlags::COMPUTE_IMPULSES);
            };

        let user_vel = self.velocities.get(user_entity).unwrap();
        let standing = user_vel.linvel.y < 0.0;

        if standing {
            Some(SolverFlags::COMPUTE_IMPULSES)
        } else {
            None
        }
    }
}

#[derive(Event)]
pub struct CrashEvent {
    pub object: Entity,
    pub force: f32,
}

#[derive(Event)]
pub struct TouchEvent(pub Entity);

pub fn write_crash(
    query_health: Query<&Health>,
    // mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut writer_crash: EventWriter<CrashEvent>,
) {
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
    mut q_h: Query<&mut Health>,
) {
    for crash in reader.read() {
        let sufferer = crash.object;
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
            writer.write(MessageEvent::info(sign.0.as_str()));
        }
    }
}

pub struct Portal;

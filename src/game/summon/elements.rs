use super::*;
use crate::game::mob::health::*;

const COMMON_GROUP: SolverGroups =
    SolverGroups::new(Group::from_bits_retain(6), Group::from_bits_retain(6));

pub fn insert_middle_ball(ec: &mut EntityCommands) {
    ec.insert((RigidBody::Dynamic, Collider::ball(7.5), COMMON_GROUP));
}

pub fn insert_middle_arrow(ec: &mut EntityCommands) {
    ec.insert((RigidBody::Dynamic, Collider::cuboid(6.5, 0.5), COMMON_GROUP));
}

pub fn insert_fire(ec: &mut EntityCommands) {
    ec.insert((
        ColliderMassProperties::Density(1.0),
        ActiveEvents::CONTACT_FORCE_EVENTS,
        LockedAxes::ROTATION_LOCKED,
        GravityScale(0.0),
        Health::fragile(),
    ));
}

pub fn insert_ice(ec: &mut EntityCommands) {
    ec.insert((
        ColliderMassProperties::Density(1000.0),
        ActiveEvents::CONTACT_FORCE_EVENTS,
        Health::fresh(0.3),
    ));
}

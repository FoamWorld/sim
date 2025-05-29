use super::*;

pub fn insert_middle_ball(ec: &mut EntityCommands) {
    ec.insert((RigidBody::Dynamic, Collider::ball(6.0)));
}

pub fn insert_fire(ec: &mut EntityCommands) {
    ec.insert((
        ColliderMassProperties::Density(1.0),
        ActiveEvents::CONTACT_FORCE_EVENTS,
        LockedAxes::ROTATION_LOCKED,
        GravityScale(0.01),
        crate::game::mob::health::Health::fragile(),
    ));
}

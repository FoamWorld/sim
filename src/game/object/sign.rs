use super::*;
use crate::{game::object::Object, physics::collision::BackgroundLevel};

#[derive(Reflect, Component, Clone)]
#[reflect(Object, Component)]
#[type_path = "sim::object"]
pub struct SignStand(usize);

impl Object for SignStand {
    fn texture_info(&self) -> Option<(&str, Option<usize>)> {
        Some(("sign", Some(self.0)))
    }

    fn add_physics_components(&self, commands: &mut EntityCommands) {
        commands.insert((
            BackgroundLevel,
            RigidBody::Dynamic,
            RigidBodyDisabled,
            Collider::rectangle(28.0, 26.0),
            Mass(40.0),
        ));
    }
}

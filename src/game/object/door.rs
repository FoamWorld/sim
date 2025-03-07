use super::*;
use crate::{game::object::Object, physics::collision::BackgroundLevel};

#[derive(Reflect, Component, Clone)]
#[reflect(Object, Component)]
#[type_path = "sim::object"]
pub struct Door {
    is_open: bool,
}

impl Object for Door {
    fn texture_info(&self) -> Option<(&str, Option<usize>)> {
        Some(("door", Some(if self.is_open { 0 } else { 1 })))
    }

    fn add_physics_components(&self, commands: &mut EntityCommands) {
        commands.insert((
            BackgroundLevel,
            RigidBody::Dynamic,
            RigidBodyDisabled,
            Collider::rectangle(16.0, 32.0),
            Mass(10.0),
        ));
    }
}

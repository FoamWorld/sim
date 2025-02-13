use crate::assets::RpgTextures;
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use std::sync::Arc;

/// Trait for implementing how a game object works.
#[diagnostic::on_unimplemented(message = "`{Self}` is not an `Object`", label = "invalid `Object`")]
pub trait Object {
    fn add_physics_components(&self, _commands: &mut EntityCommands) {}
    fn add_visual_components(&self, commands: &mut EntityCommands, rpg_folder: &Res<RpgTextures>);
    fn add_extra_components(&self, commands: &mut EntityCommands) {}
}

/// Added when the entity is a game object.
#[derive(Component)]
pub struct IsObject(Arc<dyn Object + Send + Sync>);

impl IsObject {
    fn copy_pointer(&self) -> Arc<dyn Object + Send + Sync> {
        self.0.clone()
    }
}

/*  List of objects.  */

#[derive(Component)]
pub struct Barrier {
    x_length: Scalar,
    y_length: Scalar,
}

impl Object for Barrier {
    fn add_physics_components(&self, commands: &mut EntityCommands) {
        commands.insert((
            RigidBody::Dynamic,
            Collider::rectangle(self.x_length, self.y_length),
        ));
    }
    fn add_visual_components(&self, commands: &mut EntityCommands, _: &Res<RpgTextures>) {
        commands.insert(Sprite::from_color(
            bevy::color::palettes::basic::GRAY,
            Vec2::new(self.x_length, self.y_length),
        ));
    }
}

/// A type that records unclassified objects but gives a type name.
/// Such objects do not have special effects.
/// It is suggested to use Rust type name naming rule.
#[derive(Component)]
pub struct NonUnique(pub String);

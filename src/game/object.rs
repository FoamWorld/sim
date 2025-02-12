use crate::assets::RpgTextures;
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

/// Trait for implementing how a game object works.
#[diagnostic::on_unimplemented(message = "`{Self}` is not an `Object`", label = "invalid `Object`")]
pub trait Object {
    fn add_physics_components(&self, _commands: &mut EntityCommands) {}
    fn add_sprite_components(&self, commands: &mut EntityCommands, rpg_folder: &Res<RpgTextures>);
    fn add_extra_components(&self, commands: &mut EntityCommands) {}
    // fn_spawn()
}

/// Added when the entity is a game object.
/// A struct of type `T` shall also be held.
/// Uses zero-cost abstraction.
#[derive(Component)]
pub struct ObjectType<T: Object> {
    marker: std::marker::PhantomData<T>,
}

impl<T: Object> ObjectType<T> {
    pub fn new() -> Self {
        Self {
            marker: std::marker::PhantomData::<T>::default(),
        }
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
    fn add_sprite_components(&self, commands: &mut EntityCommands, _: &Res<RpgTextures>) {
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

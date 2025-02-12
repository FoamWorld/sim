use avian2d::math::*;
use bevy::prelude::*;

/// Trait for implementing how a game object works.
trait Object {
    fn spawn_sprite() -> Option<Sprite>;
    // fn_spawn()
}

/// Added when the entity is a game object.
/// A struct of type `T` shall also be held.
/// Uses zero-cost abstraction.
#[derive(Component)]
pub struct ObjectType<T: Object> {
    marker: std::marker::PhantomData<T>,
}

#[derive(Component)]
pub struct Barrier {
    x_length: Scalar,
    y_length: Scalar,
}

/// A type that records unclassified objects but gives a type name.
/// Such objects do not have special effects.
/// It is suggested to use Rust type name naming rule.
#[derive(Component)]
pub struct NonUnique(pub String);

use crate::assets::RpgTextures;
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use std::any::TypeId;

pub struct AdditionConfig {
    // pub attached_tags: bool,
    pub physics: bool,
    pub visual: bool,
    pub extra: bool,
}

impl AdditionConfig {
    pub const FULL: Self = Self {
        physics: true,
        visual: true,
        extra: true,
    };

    pub const PLAIN_SPRITE: Self = Self {
        physics: false,
        visual: false,
        extra: false,
    };

    pub const CHARACTER_ATTACH: Self = Self {
        physics: false,
        visual: true,
        extra: true,
    };
}

/// Trait for implementing how a game object works.
#[diagnostic::on_unimplemented(message = "`{Self}` is not an `Object`", label = "invalid `Object`")]
#[reflect_trait]
pub trait Object {
    /// Mechanics when it's a physical concrete entity.
    fn add_physics_components(&self, _commands: &mut EntityCommands) {}

    /// Mechanics for **plain** visual effects.
    fn add_visual_components(&self, commands: &mut EntityCommands, rpg_folder: &RpgTextures);

    /// Mechanics when it's active.
    fn add_extra_components(&self, _commands: &mut EntityCommands) {}
}

/// Added when the entity is a game object.
#[derive(Component, Clone)]
pub struct IsObject(pub TypeId);

impl IsObject {
    pub fn add_components(
        &self,
        world: &World,
        ec: &mut EntityCommands,
        entity: Entity,
        type_registry: &AppTypeRegistry,
        config: AdditionConfig,
    ) {
        let comp = world.get_reflect(entity, self.0).unwrap();
        let guard = type_registry.0.read();
        let refl = guard.get_type_data::<ReflectObject>(self.0).unwrap();
        let obj = refl.get(&*comp).unwrap();
        if config.physics {
            obj.add_physics_components(ec);
        }
        if config.visual {
            let rpg_folder = world.get_resource::<RpgTextures>().unwrap();
            obj.add_visual_components(ec, rpg_folder);
        }
        if config.extra {
            obj.add_extra_components(ec);
        }
    }
}

/*  List of objects.  */

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Object, Component)]
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
    fn add_visual_components(&self, commands: &mut EntityCommands, _: &RpgTextures) {
        commands.insert(Sprite::from_color(
            bevy::color::palettes::basic::GRAY,
            Vec2::new(self.x_length, self.y_length),
        ));
    }
}

/// A type that records unclassified objects but gives a type name.
/// Such objects do not have special effects.
/// The sprite image will be automatically looked up.
#[derive(Reflect, Component, Clone)]
#[reflect(Object, Component)]
pub struct NonUnique(pub String);

impl Object for NonUnique {
    fn add_visual_components(&self, commands: &mut EntityCommands, rpg_folder: &RpgTextures) {
        let str = self.0.as_str();
        commands.insert(Sprite::from_image(rpg_folder.get_image_handle(str)));
    }
}

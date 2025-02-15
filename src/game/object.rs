use crate::assets::RpgTextures;
use avian2d::{math::*, prelude::*};
use bevy::{prelude::*, reflect::FromType};
use std::any::TypeId;

/// Trait for implementing how a game object works.
#[diagnostic::on_unimplemented(message = "`{Self}` is not an `Object`", label = "invalid `Object`")]
#[reflect_trait]
pub trait Object {
    fn add_physics_components(&self, _commands: &mut EntityCommands) {}
    fn add_visual_components(&self, commands: &mut EntityCommands, rpg_folder: &Res<RpgTextures>);
    fn add_extra_components(&self, _commands: &mut EntityCommands) {}
}

/// Added when the entity is a game object.
#[derive(Component, Clone)]
pub struct IsObject(pub TypeId);

impl IsObject {
    pub fn spawn_into(
        &self,
        world: &World,
        commands: &mut Commands,
        entity: Entity,
        rpg_folder: &Res<RpgTextures>,
    ) {
        let x = world.get_reflect(entity, self.0).unwrap();
        let r: ReflectObject = FromType::<Barrier>::from_type();
        let e = r.get(&*x).unwrap();
        let mut ec = commands.entity(entity);
        e.add_physics_components(&mut ec);
        e.add_visual_components(&mut ec, rpg_folder);
        e.add_extra_components(&mut ec);
    }
    pub fn spawn_attach_image(
        &self,
        world: &World,
        ec: &mut EntityCommands,
        entity: Entity,
        rpg_folder: &Res<RpgTextures>,
        type_registry: &AppTypeRegistry,
    ) {
        let x = world.get_reflect(entity, self.0).unwrap();
        let binding = type_registry.0.read();
        let r = binding.get_type_data::<ReflectObject>(self.0).unwrap();
        let e = r.get(&*x).unwrap();
        e.add_visual_components(ec, rpg_folder);
        e.add_extra_components(ec);
    }
}

/*  List of objects.  */

#[derive(Reflect)]
#[reflect(Object)]
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

#[derive(Reflect)]
#[reflect(Object)]
pub struct NonUnique(pub String);

impl Object for NonUnique {
    fn add_visual_components(&self, commands: &mut EntityCommands, rpg_folder: &Res<RpgTextures>) {
        let str = self.0.as_str();
        commands.insert(Sprite::from_image(rpg_folder.get_image_handle(str)));
    }
}

use crate::assets::RpgTextures;
use avian2d::{math::*, prelude::*};
use bevy::{
    prelude::*,
    reflect::{
        serde::{
            DeserializeWithRegistry, ReflectDeserializeWithRegistry, ReflectSerializeWithRegistry,
            SerializeWithRegistry,
        },
        TypeRegistry,
    },
    sprite::Anchor,
};
use serde::{de::Visitor, Serializer};
use std::any::TypeId;

pub enum VisualType {
    Simple,
    Grid,
    Active,
}

pub struct AdditionConfig {
    // pub attached_tags: bool,
    pub physics: bool,
    pub visual: Option<VisualType>,
    pub extra: bool,
}

impl AdditionConfig {
    pub const IN_SCENE: Self = Self {
        physics: true,
        visual: Some(VisualType::Simple),
        extra: false,
    };

    pub const IN_GRID: Self = Self {
        physics: false,
        visual: Some(VisualType::Grid),
        extra: false,
    };

    pub const CHARACTER_ATTACH: Self = Self {
        physics: false,
        visual: Some(VisualType::Active),
        extra: true,
    };
}

/// Trait for implementing how a game object works.
#[diagnostic::on_unimplemented(message = "`{Self}` is not an `Object`", label = "invalid `Object`")]
#[reflect_trait]
pub trait Object {
    fn texture_info(&self) -> Option<(&str, Option<usize>)> {
        None
    }

    fn texture_anchor(&self) -> Anchor {
        Anchor::Center
    }

    /// Mechanics when it's a physical concrete entity.
    fn add_physics_components(&self, _commands: &mut EntityCommands) {}

    /// Mechanics for **plain** visual effects.
    fn add_visual_components(
        &self,
        commands: &mut EntityCommands,
        rpg_folder: &RpgTextures,
        visual_type: VisualType,
    ) {
        let (image, size) = self.texture_info().unwrap();
        match visual_type {
            VisualType::Simple => {
                commands.insert(Sprite {
                    image: rpg_folder.get_image_handle(image),
                    texture_atlas: size.and_then(|x| Some(rpg_folder.get_texture_atlas(image, x))),
                    ..default()
                });
            }
            VisualType::Grid => {
                commands.insert(ImageNode {
                    image: rpg_folder.get_image_handle(image),
                    texture_atlas: size.and_then(|x| Some(rpg_folder.get_texture_atlas(image, x))),
                    ..default()
                });
            }
            VisualType::Active => {
                commands.insert(Sprite {
                    image: rpg_folder.get_image_handle(image),
                    texture_atlas: size.and_then(|x| Some(rpg_folder.get_texture_atlas(image, x))),
                    anchor: self.texture_anchor(),
                    ..default()
                });
            }
        }
    }

    /// Mechanics when it's active.
    fn add_extra_components(&self, _commands: &mut EntityCommands) {}
}

/// Added when the entity is a game object.
#[derive(Reflect, Component, Clone)]
#[reflect(Component)]
#[reflect(SerializeWithRegistry, DeserializeWithRegistry)]
#[component(storage = "Table")]
#[type_path = "sim::core"]
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
        if let Some(visual_type) = config.visual {
            let rpg_folder = world.get_resource::<RpgTextures>().unwrap();
            obj.add_visual_components(ec, rpg_folder, visual_type);
        }
        if config.extra {
            obj.add_extra_components(ec);
        }
    }
}

impl SerializeWithRegistry for IsObject {
    fn serialize<S>(&self, serializer: S, registry: &TypeRegistry) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let registeration = registry.get(self.0).unwrap();
        let info = registeration.type_info();
        let path = info.type_path();
        let state = serializer.serialize_str(path);
        state
    }
}

impl<'de> DeserializeWithRegistry<'de> for IsObject {
    fn deserialize<D>(deserializer: D, registry: &TypeRegistry) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MyVisitor<'a> {
            registry: &'a TypeRegistry,
        }

        impl<'a, 'de> Visitor<'de> for MyVisitor<'a> {
            type Value = TypeId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a type path")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                let registeration = self.registry.get_with_type_path(v).unwrap();
                let id = registeration.type_id();
                Ok(id)
            }
        }

        deserializer
            .deserialize_str(MyVisitor { registry })
            .map(IsObject)
    }
}

/*  List of objects.  */

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Object, Component)]
#[type_path = "sim::object"]
pub struct Barrier {
    pub x_length: Scalar,
    pub y_length: Scalar,
}

impl Object for Barrier {
    fn add_physics_components(&self, commands: &mut EntityCommands) {
        commands.insert((
            RigidBody::Static,
            Collider::rectangle(self.x_length, self.y_length),
        ));
    }
    fn add_visual_components(&self, commands: &mut EntityCommands, _: &RpgTextures, _: VisualType) {
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
#[type_path = "sim::object"]
pub struct NonUnique(pub String);

impl Object for NonUnique {
    fn texture_info(&self) -> Option<(&str, Option<usize>)> {
        Some((self.0.as_str(), None))
    }
}

pub mod sign;

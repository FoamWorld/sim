use crate::assets::RpgTextures;
use crate::constants::*;
use bevy::prelude::*;

/* Relation */

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::utils"]
#[relationship(relationship_target = Eidos)]
pub struct Methexis(pub Entity);

#[derive(Reflect, Component, Clone, Deref)]
#[reflect(Component)]
#[type_path = "sim::utils"]
#[relationship_target(relationship = Methexis)]
pub struct Eidos(Vec<Entity>);

/* Components */

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::utils"]
pub enum HoldsConfig {
    Default,
    Wand,
    Custom((Vec2, (Scalar, Scalar))),
}

impl HoldsConfig {
    pub fn get_offset(&self) -> Vec2 {
        match self {
            HoldsConfig::Default => Vec2::ZERO,
            HoldsConfig::Wand => Vec2::new(-0.4, 0.0),
            HoldsConfig::Custom((vec, _)) => *vec,
        }
    }

    pub fn get_rotate_range(&self) -> (Scalar, Scalar) {
        match self {
            HoldsConfig::Default => (0.0, 0.0),
            HoldsConfig::Wand => (0.0, PI * 0.4),
            HoldsConfig::Custom((_, tuple)) => *tuple,
        }
    }
}

#[derive(Reflect, Component, Clone)]
#[reflect(Component)]
#[type_path = "sim::utils"]
pub struct IconImage {
    pub sheet: String,
    pub index: Option<usize>,
}

impl IconImage {
    pub fn image_node(&self, rpg_folder: &RpgTextures) -> ImageNode {
        ImageNode {
            image: rpg_folder.get_image_handle(self.sheet.as_str()),
            texture_atlas: self
                .index
                .and_then(|x| Some(rpg_folder.get_texture_atlas(self.sheet.as_str(), x))),
            ..default()
        }
    }

    pub fn sprite(&self, rpg_folder: &RpgTextures) -> Sprite {
        Sprite {
            image: rpg_folder.get_image_handle(self.sheet.as_str()),
            texture_atlas: self
                .index
                .and_then(|x| Some(rpg_folder.get_texture_atlas(self.sheet.as_str(), x))),
            ..default()
        }
    }
}

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::utils"]
pub struct Mode(pub usize);

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::utils"]
pub struct PhysicsConfig {
    pub mass: Scalar,
    // center_of_mass
    pub shape: Vec2,
}

// #[derive(Component)]
// pub struct TypeHash(pub usize);

/* Interaction Commands */

#[derive(Reflect, Component)]
#[reflect(Component)]
#[type_path = "sim::utils"]
pub struct ActivateCommand {
    func: Box<dyn Fn(&mut Commands, &World, Entity, Vec2, Vec2) + Send + Sync>,
}

impl ActivateCommand {
    pub fn new(
        func: impl Fn(&mut Commands, &World, Entity, Vec2, Vec2) + Send + Sync + 'static,
    ) -> Self {
        Self {
            func: Box::new(func),
        }
    }

    pub fn execute(
        &self,
        commands: &mut Commands,
        world: &World,
        entity: Entity,
        source: Vec2,
        target: Vec2,
    ) {
        (self.func)(commands, world, entity, source, target);
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
#[type_path = "sim::utils"]
pub struct ModifyCommand {
    func: Box<dyn Fn(&mut Commands, Entity) + Send + Sync>,
}

impl ModifyCommand {
    pub fn new(func: impl Fn(&mut Commands, Entity) + Send + Sync + 'static) -> Self {
        Self {
            func: Box::new(func),
        }
    }

    pub fn execute(&self, commands: &mut Commands, entity: Entity) {
        (self.func)(commands, entity);
    }
}

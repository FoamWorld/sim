use super::object::*;
use crate::assets::RpgTextures;
use bevy::prelude::*;
use std::any::TypeId;

/// Added when the clone is inside an item container.
/// Field `.0` stores the number/amount stacked. This works for non-unique solids, liquids and gases.
/// Using `f32` is acceptable since it rarely happens someone takes 8 out of 1e10, say.
/// And when that happens, it can count as a feature.
#[derive(Component)]
pub struct ItemAmount(pub f32);

// todo: limits on weight (and volume?)
/// Component for item containers. This may include: the characters two hands (and back (and pockets?)?).
#[derive(Component, Clone)]
pub struct ItemStorage(pub Vec<Option<Entity>>);

impl ItemStorage {
    pub fn with_capacity(length: usize) -> Self {
        Self(vec![None; length])
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn get_index(&self, index: usize) -> Option<Entity> {
        self.0[index]
    }

    pub fn force_give(&mut self, commands: &mut Commands, entity: Entity) {
        self.0[0] = Some(entity);
        commands.entity(entity).insert(ItemAmount(1.0));
    }
}

/// Trait for implementing how an item works.
#[reflect_trait]
pub trait Item {
    fn check_can_use(&self) -> bool {
        true
    }
    fn item_use(
        &self,
        commands: &mut Commands,
        entity: Entity,
        source: Vec2,
        target: Option<Vec2>,
        rpg_folder: &RpgTextures,
    );
    fn check_can_modify(&self) -> bool {
        false
    }
    fn item_modify(&self, commands: &mut Commands, entity: Entity);
}

/// Added when the entity can work as an item.
/// The different status of an item entity:
/// * (unique) lying on the ground:
/// contains `#type`, `#is-tags`, `#physics`, `#visual`
/// * (unique) in storage:
/// contains `#type`, `#is-tags`, `ItemAmount (inserted)`
/// * (clone) sprite in storage display:
/// contains `#visual`
/// * (clone) active entity attached to the character:
/// contains `#visual`, `#extra (?RotateWithMouse)`
#[derive(Component, Clone)]
pub struct IsItem(pub TypeId);

impl IsItem {
    pub fn inspect_then<F>(
        &self,
        world: &World,
        entity: Entity,
        type_registry: &AppTypeRegistry,
        f: F,
    ) where
        F: FnOnce(&dyn Item) -> (),
    {
        let comp = world.get_reflect(entity, self.0).unwrap();
        let guard = type_registry.0.read();
        let refl = guard.get_type_data::<ReflectItem>(self.0).unwrap();
        let it = refl.get(&*comp).unwrap();
        f(it);
    }
}

/* List of items. */

pub mod debug_wand;

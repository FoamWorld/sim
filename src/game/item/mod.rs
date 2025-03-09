use super::object::*;
use crate::assets::RpgTextures;
use bevy::prelude::*;

/// Used in item containers.
/// Field `.0` stores the number/amount stacked. This works for non-unique solids, liquids and gases.
/// Using `f32` is acceptable since it rarely happens someone takes 8 out of 1e10, say.
/// And when that happens, it can count as a feature.
type ItemAmount = f32;

/// Component for item containers. This may include: the characters two hands (and back (and pockets?)?).
#[derive(Reflect, Component)]
#[reflect(Component)]
#[type_path = "sim::item"]
pub struct ItemStorage {
    // (?) todo: volume limit
    pub limit: f32, // weight limit
    pub count: Vec<ItemAmount>,
    pub storage: Vec<Option<Entity>>,
}

impl ItemStorage {
    pub fn with_capacity(length: usize) -> Self {
        Self {
            limit: f32::INFINITY,
            count: vec![0.0; length],
            storage: vec![None; length],
        }
    }

    pub fn size(&self) -> usize {
        self.storage.len()
    }

    pub fn view_count(&self, index: usize) -> f32 {
        if self.storage[index].is_none() {
            0.0
        } else {
            self.count[index]
        }
    }

    pub fn get_index(&self, index: usize) -> Option<Entity> {
        self.storage[index]
    }

    pub fn force_give(&mut self, index: usize, entity: Entity) {
        self.count[index] = 1.0;
        self.storage[index] = Some(entity);
    }

    pub fn extract_one(&mut self, index: usize) -> Option<Entity> {
        if let Some(entity) = self.storage[index] {
            self.count[index] -= 1.0;
            if self.count[index] < 1e-3 {
                self.storage[index] = None;
            }
            Some(entity)
        } else {
            None
        }
    }
}

/// Trait for implementing how an item works.
#[reflect_trait]
pub trait Item {
    fn activate(
        &self,
        commands: &mut Commands,
        entity: Entity,
        source: Vec2,
        target: Option<Vec2>,
        rpg_folder: &RpgTextures,
    );

    fn modify(&self, _commands: &mut Commands, _entity: Entity) {}
}

/// Added when the entity can work as an item.
/// The different status of an item entity:
/// * (unique) lying in the scene:
/// contains `#type`, `#refs`, `#physics`, `#visual`
/// * (unique) in storage (inventory, etc.):
/// contains `#type`, `#refs`
/// * (clone) sprite in storage display:
/// contains `#visual`
/// * (clone) active entity attached to the character:
/// contains `#visual`, `#extra (?RotateWithMouse)`
#[derive(Reflect, Component, Clone)]
#[reflect(Component)]
#[type_path = "sim::item"]
pub struct ItemRef;

impl ItemRef {
    pub fn apply_to_item<F>(world: &World, entity: Entity, type_registry: &AppTypeRegistry, f: F)
    where
        F: FnOnce(&dyn Item) -> (),
    {
        let id = world.entity(entity).get::<ObjectRef>().unwrap().0;
        let comp = world.get_reflect(entity, id).unwrap();
        let guard = type_registry.0.read();
        let refl = guard.get_type_data::<ReflectItem>(id).unwrap();
        let it = refl.get(&*comp).unwrap();
        f(it);
    }
}

/* List of items. */

pub mod wand;

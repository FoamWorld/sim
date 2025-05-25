use bevy::prelude::*;

/// Used in item containers.
/// Field `.0` stores the number/amount stacked. This works for non-unique solids, liquids and gases.
/// Using `f32` is acceptable since it rarely happens someone takes 8 out of 1e10, say.
/// And when that happens, it can count as a feature.
type ItemAmount = f32;

/// Component for item containers. This may include: the characters' two hands (and back (and pockets?)?).
///
/// Contains list of `Eidos` entities.
#[derive(Reflect, Component, Clone)]
#[reflect(Component)]
#[type_path = "sim::utils"]
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

    pub fn force_give(&mut self, index: usize, entity: Entity, count: ItemAmount) {
        self.count[index] = count;
        self.storage[index] = Some(entity);
    }

    pub fn more_than_one(&self, index: usize) -> bool {
        self.count[index] - 1.0 > 1e-3
    }
}

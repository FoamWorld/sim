use bevy::prelude::*;

/// Added when it works as an item.
/// When it reveals sprite, it not necessarily hides `RigidBody`.
/// Field `.0` stores the number/amount stacked. This works for non-unique solids, liquids and gases.
/// Using `f32` is acceptable since it rarely happens someone takes 8 out of 1e10, say. And when that happens, it can count as a feature.
#[derive(Component)]
struct Item(pub f32);

#[derive(Component)]
struct ItemStorage(pub Vec<Option<Entity>>);

impl ItemStorage {
    fn new(length: usize) -> Self {
        Self(vec![None; length])
    }
}

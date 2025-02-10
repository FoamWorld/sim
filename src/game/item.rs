use bevy::prelude::*;

/// Added when it works as an item.
/// When it reveals sprite, it not necessarily hides `RigidBody`.
/// Field `.0` stores the number/amount stacked. This works for non-unique solids, liquids and gases.
/// Using `f32` is acceptable since it rarely happens someone takes 8 out of 1e10, say. And when that happens, it can count as a feature.
#[derive(Component)]
pub struct Item(pub f32);

// todo: limits on weight (and volume?)
#[derive(Component)]
pub struct ItemStorage(pub Vec<Option<Entity>>);

impl ItemStorage {
    pub fn with_capacity(length: usize) -> Self {
        Self(vec![None; length])
    }

    pub fn force_give(&mut self, commands: &mut Commands, entity: Entity) {
        self.0[0] = Some(entity);
        commands.entity(entity).insert(Item(1.0));
    }
}

use super::object::Object;
use bevy::prelude::*;
use std::sync::Arc;

/// Added when the clone is inside an item container.
/// In this state, the entity spawns a `sprite` when required, but does not hold a `RigidBody`.
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
pub trait Item {
    fn feed_to_storage(&self);
    fn check_can_use(&self) -> bool {
        true
    }
    fn item_use(
        &mut self,
        commands: &mut Commands,
        hold_point: Vec2,
        coords: Res<crate::physics::camera::CursorCoords>,
        rpg_folder: Res<crate::assets::RpgTextures>,
    );
    fn check_can_consume(&self) -> bool {
        false
    }
    fn item_consume(&self);
}

/// Added when the entity can work as an item.
/// There are three different status for an item entity:
/// * lying on the ground (primary):          includes `IsObject`, `IsItem`, visual, ?physics
/// * visual image attached to the character: includes `IsObject`, `IsItem`, visual, ?physics, extra
/// * visual image in inventory (primary):    includes `IsObject`, `IsItem`, `ItemAmount`, visual
/// Uses zero-cost abstraction.
#[derive(Component)]
pub struct IsItem(pub Arc<dyn Item + Send + Sync>);

/* List of items. */

pub mod debug_wand;

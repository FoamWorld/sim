use super::object::*;
use bevy::{prelude::*, reflect::FromType};
use debug_wand::DebugWand;
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
        hold_point: Vec2,
        coords: Option<Vec2>,
        rpg_folder: Res<crate::assets::RpgTextures>,
    );
    fn check_can_consume(&self) -> bool {
        false
    }
    fn item_consume(&mut self);
}

/// Added when the entity can work as an item.
/// There are three different status for an item entity:
/// * lying on the ground (primary):
/// includes `IsObject`, `IsItem`, `#visual`, `?#physics`
/// * visual image attached to the character:
/// includes `#visual`, `#extra (?RotateWithMouse)`
/// * visual image in inventory (primary):
/// includes `IsObject`, `IsItem`, `#visual`, `ItemAmount (inserted)`
///
/// Uses zero-cost abstraction.
#[derive(Component, Clone)]
pub struct IsItem(pub TypeId);

impl IsItem {
    pub fn useable(&self, world: &World, entity: Entity) -> bool {
        let x = world.get_reflect(entity, self.0).unwrap();
        let r: ReflectItem = FromType::<DebugWand>::from_type();
        let e = r.get(&*x).unwrap();
        e.check_can_use()
    }
    pub fn item_use(
        &self,
        commands: &mut Commands,
        world: &World,
        entity: Entity,
        hold_point: Vec2,
        coords: Option<Vec2>,
        rpg_folder: Res<crate::assets::RpgTextures>,
    ) {
        let x = world.get_reflect(entity, self.0).unwrap();
        let r: ReflectItem = FromType::<DebugWand>::from_type();
        let e = r.get(&*x).unwrap();
        e.item_use(commands, entity, hold_point, coords, rpg_folder);
    }
}

/* List of items. */

pub mod debug_wand;

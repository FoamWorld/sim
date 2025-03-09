use super::{inventory::*, item::*, object::*};
use crate::{assets::*, character::*};
use bevy::prelude::*;

fn reach_inventory_item_then<F>(world: &World, inv: Entity, index: usize, f: F)
where
    F: FnOnce(&dyn Item, Entity) -> (),
{
    let storage = world.entity(inv).get::<ItemStorage>().unwrap();
    let item = if let Some(item) = storage.get_index(index) {
        item
    } else {
        return;
    };

    let type_registry = world.resource::<AppTypeRegistry>();
    ItemRef::apply_to_item(world, item, type_registry, |guarded| {
        f(guarded, item);
    });
}

pub fn item_use(mut commands: Commands, world: &World, inventory: Res<Inventory>) {
    let inv = inventory.bind.unwrap();
    let index = inventory.selected;

    let rpg_folder = world.resource::<RpgTextures>();

    let position = world.resource::<ActorPosition>();
    let target = world.resource::<crate::physics::camera::CursorCoords>();
    reach_inventory_item_then(world, inv, index, |guarded, item| {
        guarded.activate(
            &mut commands,
            item,
            position.center + position.primary_hand_offset,
            target.0,
            rpg_folder,
        );
    });
}

pub fn item_modify(mut commands: Commands, world: &World, inventory: Res<Inventory>) {
    let inv = inventory.bind.unwrap();
    let index = inventory.selected;
    reach_inventory_item_then(world, inv, index, |guarded, item| {
        guarded.modify(&mut commands, item);
    });
}

pub fn item_throw(mut commands: Commands, world: &World, inventory: Res<Inventory>) {
    let inv = inventory.bind.unwrap();
    let storage = world.entity(inv).get::<ItemStorage>().unwrap();
    let index = inventory.selected;
    if storage.view_count(index) < 1.0 {
        return;
    }

    commands
        .entity(inv)
        .queue(move |mut entity: EntityWorldMut| {
            let mut storage = entity.get_mut::<ItemStorage>().unwrap();
            storage.extract_one(index.clone());
        });

    let it = world
        .entity(storage.get_index(index).unwrap())
        .get::<ObjectRef>()
        .unwrap();
}

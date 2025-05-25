use super::{ecs::*, inventory::*, item_storage::*};
use crate::character::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn reach_inventory_item(world: &World, inv: Entity, index: usize) -> Option<Entity> {
    let storage = world.entity(inv).get::<ItemStorage>().unwrap();
    storage.get_index(index)
}

fn get_source_target(world: &World) -> (Vec2, Vec2) {
    let position = world.resource::<ActorPosition>();
    let source = position.center + position.primary_hand_offset;

    let cursor_coords = world.resource::<crate::physics::camera::CursorCoords>();
    let target = cursor_coords.0.unwrap_or(source + position.facing_offset);

    (source, target)
}

pub fn item_use(mut commands: Commands, world: &World, inventory: Res<Inventory>) {
    let inv = inventory.bind.unwrap();
    let index = inventory.selected;

    let (source, target) = get_source_target(world);

    if let Some(item) = reach_inventory_item(world, inv, index) {
        if let Some(cmd) = world.entity(item).get::<ActivateCommand>() {
            cmd.execute(&mut commands, world, item, source, target);
        }
    }
}

pub fn item_modify(mut commands: Commands, world: &World, inventory: Res<Inventory>) {
    let inv = inventory.bind.unwrap();
    let index = inventory.selected;

    if let Some(item) = reach_inventory_item(world, inv, index) {
        if let Some(cmd) = world.entity(item).get::<ModifyCommand>() {
            cmd.execute(&mut commands, item);
        }
    }
}

pub fn item_throw(mut commands: Commands, world: &World, inventory: Res<Inventory>) {
    let inv = inventory.bind.unwrap();
    let storage = world.entity(inv).get::<ItemStorage>().unwrap();
    let index = inventory.selected;
    if storage.view_count(index) < 1.0 {
        return;
    }

    // Use existing `Entity` to avoid panic.
    let chosen = storage.storage[index].unwrap();
    if storage.more_than_one(index) {
        commands
            .entity(inv)
            .queue(move |mut entity: EntityWorldMut| {
                let mut storage = entity.get_mut::<ItemStorage>().unwrap();
                storage.count[index.clone()] -= 1.0;
            });

        commands.entity(chosen).clone_and_spawn();
    } else {
        commands
            .entity(inv)
            .queue(move |mut entity: EntityWorldMut| {
                let mut storage = entity.get_mut::<ItemStorage>().unwrap();
                storage.storage[index.clone()] = None;
            });
    };

    let (source, target) = get_source_target(world);
    let ec = commands.spawn((
        Methexis(chosen),
        Transform::from_translation(source.extend(0.0)),
        Velocity::linear(target - source),
    ));
    super::feed::feed_to_concrete(chosen, world, ec);
}

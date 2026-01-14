use super::{ecs::*, inventory::*, item_storage::*};
use crate::character::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(EntityEvent)]
pub struct Activation {
    pub entity: Entity,
    pub pointing: ItemPointing,
}

#[derive(EntityEvent)]
pub struct Communication(pub Entity);

#[derive(Clone, Copy)]
pub struct ItemPointing {
    pub source: Vec2,
    pub target: Vec2,
    pub rotation: f32,
}

fn reach_inventory_item(world: &World, inv: Entity, index: usize) -> Option<Entity> {
    let storage = world.entity(inv).get::<ItemStorage>().unwrap();
    storage.get_index(index)
}

fn get_target(world: &World, source: &Vec2) -> Vec2 {
    let status = world.resource::<ActorStatus>();
    let cursor_coords = world.resource::<crate::physics::camera::CursorCoords>();
    cursor_coords.0.unwrap_or(source + status.facing_offset)
}

pub fn item_use(
    mut commands: Commands,
    world: &World,
    inventory: Res<Inventory>,
    transform: &GlobalTransform,
) {
    let inv = inventory.bind.unwrap();
    let index = inventory.selected;

    let source = transform.translation().truncate();
    let target = get_target(world, &source);

    if let Some(item) = reach_inventory_item(world, inv, index) {
        let rot = transform.rotation();
        let pointing = ItemPointing {
            source,
            target,
            rotation: ops::atan2(rot.z, rot.w) * 2.0,
        };
        commands.trigger(Activation {
            entity: item,
            pointing: pointing,
        });
    }
}

pub fn item_modify(mut commands: Commands, world: &World, inventory: Res<Inventory>) {
    let inv = inventory.bind.unwrap();
    let index = inventory.selected;

    if let Some(item) = reach_inventory_item(world, inv, index) {
        commands.trigger(Communication(item));
    }
}

pub fn item_throw(
    mut commands: Commands,
    world: &World,
    inventory: Res<Inventory>,
    transform: &GlobalTransform,
) {
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

    let source = transform.translation().truncate();
    let target = get_target(world, &source);
    let ec = commands.spawn((
        Methexis(chosen),
        Transform::from_translation(source.extend(0.0)),
        Velocity::linear(target - source),
        SolverGroups::new(Group::GROUP_2, Group::GROUP_2),
    ));
    super::feed::feed_to_concrete(chosen, world, ec);
}

pub fn item_pick(mut commands: Commands, world: &World, hover: Entity, inventory: Res<Inventory>) {
    let entity = world.entity(hover);
    if let Some(methexis) = entity.get::<Methexis>() {
        let bind = inventory.bind.unwrap();
        if let Some(index) = world.entity(bind).get::<ItemStorage>().unwrap().free_slot() {
            let eidos = methexis.0;
            commands.entity(hover).despawn();
            commands
                .entity(bind)
                .entry::<ItemStorage>()
                .and_modify(move |mut storage| {
                    storage.force_give(index, eidos, 1.0);
                });
        }
    }
}

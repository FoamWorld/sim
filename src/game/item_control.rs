use super::{ecs::*, inventory::*, item_storage::*};
use crate::character::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Reflect, Component)]
#[reflect(Component)]
#[type_path = "sim::utils"]
pub struct ActivateCommand {
    func: Box<dyn Fn(&mut Commands, &World, Entity, ItemPointing) + Send + Sync>,
}

impl ActivateCommand {
    pub fn new(
        func: impl Fn(&mut Commands, &World, Entity, ItemPointing) + Send + Sync + 'static,
    ) -> Self {
        Self {
            func: Box::new(func),
        }
    }

    pub fn execute(
        &self,
        commands: &mut Commands,
        world: &World,
        entity: Entity,
        pointing: ItemPointing,
    ) {
        (self.func)(commands, world, entity, pointing);
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
#[type_path = "sim::utils"]
pub struct ModifyCommand {
    func: Box<dyn Fn(&mut Commands, Entity) + Send + Sync>,
}

impl ModifyCommand {
    pub fn new(func: impl Fn(&mut Commands, Entity) + Send + Sync + 'static) -> Self {
        Self {
            func: Box::new(func),
        }
    }

    pub fn execute(&self, commands: &mut Commands, entity: Entity) {
        (self.func)(commands, entity);
    }
}

pub struct ItemPointing {
    pub source: Vec2,
    pub target: Vec2,
    pub rotation: f32,
}

fn reach_inventory_item(world: &World, inv: Entity, index: usize) -> Option<Entity> {
    let storage = world.entity(inv).get::<ItemStorage>().unwrap();
    storage.get_index(index)
}

fn get_source_target(world: &World) -> (Vec2, Vec2) {
    let position = world.resource::<ActorStatus>();
    let source = position.center + position.primary_hand_offset;

    let cursor_coords = world.resource::<crate::physics::camera::CursorCoords>();
    let target = cursor_coords.0.unwrap_or(source + position.facing_offset);

    (source, target)
}

pub fn item_use(
    mut commands: Commands,
    world: &World,
    inventory: Res<Inventory>,
    transform: &GlobalTransform,
) {
    let inv = inventory.bind.unwrap();
    let index = inventory.selected;

    let (source, target) = get_source_target(world);

    if let Some(item) = reach_inventory_item(world, inv, index) {
        let rot = transform.rotation();
        let pointing = ItemPointing {
            source,
            target,
            rotation: ops::atan2(rot.z, rot.w) * 2.0,
        };
        if let Some(cmd) = world.entity(item).get::<ActivateCommand>() {
            cmd.execute(&mut commands, world, item, pointing);
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

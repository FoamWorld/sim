use crate::constants::*;
use bevy::ecs::component::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod barrier;
pub mod cloth;
pub mod door;
pub mod platform;
pub mod sign;

#[derive(Reflect, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct Position(pub Scalar, pub Scalar, pub Scalar);

impl Component for Position {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, context| {
            let (x, y, z) = {
                let pos = world.entity(context.entity).get::<Position>().unwrap();
                (pos.0, pos.1, pos.2)
            };
            let mut binding = world.commands();
            let mut commands = binding.entity(context.entity);
            commands.insert(Transform::from_xyz(x, y, z));
            commands.remove::<Position>();
        })
    }
}

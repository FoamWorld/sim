use super::ecs::*;
use bevy::ecs::component::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod wand;

#[derive(Reflect, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct LainItemModel;

impl Component for LainItemModel {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, context| {
            let mut binding = world.commands();
            let mut commands = binding.entity(context.entity);
            commands.insert(SolverGroups::new(Group::GROUP_2, Group::GROUP_2));
            commands.remove::<LainItemModel>();
        })
    }
}

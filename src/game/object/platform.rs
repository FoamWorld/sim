use super::*;
use bevy_tnua::TnuaGhostPlatform;

#[derive(Reflect, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct ShelfModel {
    pub half_x: Scalar,
}

impl Component for ShelfModel {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, context| {
            let half_x = world
                .entity(context.entity)
                .get::<ShelfModel>()
                .unwrap()
                .half_x;
            let mut binding = world.commands();
            let mut commands = binding.entity(context.entity);
            commands.insert((
                Sprite::from_color(Color::srgb(0.8, 0.6, 0.6), Vec2::new(2.0 * half_x, 1.0)),
                RigidBody::Fixed,
                SolverGroups::new(Group::GROUP_3, Group::GROUP_2 | Group::GROUP_3),
                Collider::cuboid(half_x, 0.5),
                TnuaGhostPlatform,
            ));
            commands.remove::<ShelfModel>();
        })
    }
}

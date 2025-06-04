use super::*;
use crate::constants::Scalar;

#[derive(Reflect, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct BarrierModel {
    pub half_x: Scalar,
    pub half_y: Scalar,
    pub color: (f32, f32, f32, f32),
}

impl BarrierModel {
    pub fn new(half_x: Scalar, half_y: Scalar, color: (f32, f32, f32, f32)) -> Self {
        Self {
            half_x,
            half_y,
            color,
        }
    }
}

impl Component for BarrierModel {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, context| {
            let (half_x, half_y, (red, green, blue, alpha)) = {
                let model = world.entity(context.entity).get::<BarrierModel>().unwrap();
                (model.half_x, model.half_y, model.color)
            };
            let mut binding = world.commands();
            let mut commands = binding.entity(context.entity);
            commands.insert((
                Sprite::from_color(
                    Color::srgba(red, green, blue, alpha),
                    Vec2::new(half_x * 2.0, half_y * 2.0),
                ),
                RigidBody::Fixed,
                Collider::cuboid(half_x, half_y),
            ));
            commands.remove::<BarrierModel>();
        })
    }
}

#[derive(Reflect, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct PlatformRoomModel {
    pub h: Scalar,
    pub v: Scalar,
    pub r: Scalar,
}

impl Component for PlatformRoomModel {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, context| {
            let (h, v, r) = {
                let model = world
                    .entity(context.entity)
                    .get::<PlatformRoomModel>()
                    .unwrap();
                (model.h, model.v, model.r)
            };
            let mut binding = world.commands();
            let mut commands = binding.entity(context.entity);
            let transparent = (0.0, 0.0, 0.0, 0.0);
            let silver = (0.75, 0.75, 0.75, 1.0);
            commands.insert(Visibility::Visible);
            commands.insert(children![
                (
                    BarrierModel::new(h, r, silver),
                    Transform::from_xyz(0.0, -v, 0.0),
                ),
                (
                    BarrierModel::new(h, r, transparent),
                    Transform::from_xyz(0.0, v, 0.0),
                ),
                (
                    BarrierModel::new(r, v - r, transparent),
                    Transform::from_xyz(h, 0.0, 0.0),
                ),
                (
                    BarrierModel::new(r, v - r, transparent),
                    Transform::from_xyz(-h, 0.0, 0.0),
                )
            ]);
            commands.remove::<PlatformRoomModel>();
        })
    }
}

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
pub struct WallModel {
    pub h: (Scalar, Scalar),
    pub v: (Scalar, Scalar),
}

impl Component for WallModel {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, context| {
            let ((hl, hr), (vl, vr)) = {
                let model = world.entity(context.entity).get::<WallModel>().unwrap();
                (model.h, model.v)
            };
            let half_x = (hr - hl) * 0.5;
            let half_y = (vr - vl) * 0.5;
            let mut binding = world.commands();
            let mut commands = binding.entity(context.entity);
            commands.insert((
                Visibility::Visible,
                Transform::from_xyz((hl + hr) * 0.5, (vl + vr) * 0.5, 0.0),
                Sprite::from_color(Color::WHITE, Vec2::new(half_x * 2.0, half_y * 2.0)),
                RigidBody::Fixed,
                Collider::cuboid(half_x, half_y),
            ));
            commands.remove::<WallModel>();
        })
    }
}

#[derive(Reflect, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct GroundModel {
    pub norm: Vec2,
}
impl Component for GroundModel {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, context| {
            let outward_normal = {
                let model = world.entity(context.entity).get::<GroundModel>().unwrap();
                model.norm
            };
            let mut binding = world.commands();
            let mut commands = binding.entity(context.entity);
            commands.insert((
                RigidBody::Fixed,
                Collider::halfspace(outward_normal).unwrap(),
            ));
            commands.remove::<GroundModel>();
        })
    }
}

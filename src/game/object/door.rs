use super::*;

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::utils"]
pub struct Open(pub bool);

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::utils"]
pub enum Lock {
    None,
    // Password(String),
    // KeySlot(?),
}

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::utils"]
pub struct SideDoorStatus {
    is_open: bool,
    open_extend: Scalar, // +/-, extends
}

#[derive(Reflect, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct SideDoorModel {
    h: (Scalar, Scalar),
    hr: Scalar,
    v: (Scalar, Scalar),
}

impl Component for SideDoorModel {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, context| {
            let ((hl, hr), (vl, vr)) = {
                let model = world.entity(context.entity).get::<SideDoorModel>().unwrap();
                (model.h, model.v)
            };
            let half_x = (hr - hl) * 0.5;
            let half_y = (vr - vl) * 0.5;
            let mut binding = world.commands();
            let mut commands = binding.entity(context.entity);
            commands.insert((
                Visibility::Visible,
                Transform::from_xyz((hl + hr) * 0.5, (vl + vr) * 0.5, 0.0),
                Sprite::from_color(
                    Color::srgb(0.7, 0.7, 0.7),
                    Vec2::new(half_x * 2.0, half_y * 2.0),
                ),
                RigidBody::Fixed,
                Collider::cuboid(half_x, half_y),
                Lock::None,
            ));
            commands.remove::<SideDoorModel>();
        })
    }
}

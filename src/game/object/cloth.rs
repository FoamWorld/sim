use super::*;
use crate::constants::Scalar;

#[derive(Reflect, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct ClothModel {
    pub h: Scalar,
    pub v: Scalar,
    pub c: (Scalar, Scalar, Scalar),
}

impl Component for ClothModel {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, context| {
            let (h, v, (r, g, b)) = {
                let model = world.entity(context.entity).get::<ClothModel>().unwrap();
                (model.h, model.v, model.c)
            };
            let mut binding = world.commands();
            let mut commands = binding.entity(context.entity);
            commands.insert(children![(
                Transform::from_xyz(0.0, 0.0, -2.0),
                Sprite::from_color(Color::srgb(r, g, b), Vec2::new(2.0 * h, 2.0 * v)),
            )]);
            commands.remove::<ClothModel>();
        })
    }
}

use super::*;
use crate::constants::Scalar;

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct ClothModel {
    pub h: Scalar,
    pub v: Scalar,
    pub c: (Scalar, Scalar, Scalar),
}

pub fn setup_cloth_model(
    mut commands: Commands,
    query: Query<(Entity, &ClothModel), Added<ClothModel>>,
) {
    for (entity, model) in &query {
        let (r, g, b) = model.c;
        commands.entity(entity).insert(children![(
            Visibility::Visible,
            Transform::from_xyz(0.0, 0.0, 0.0),
            Sprite::from_color(
                Color::srgb(r, g, b),
                Vec2::new(2.0 * model.h, 2.0 * model.v)
            ),
        )]);
    }
}

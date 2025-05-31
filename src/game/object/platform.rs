use super::*;
use crate::physics::collision::OneWayPlatform;

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct ShelfModel {
    pub half_x: Scalar,
}

pub fn setup_shelf_model(
    mut commands: Commands,
    query: Query<(Entity, &ShelfModel), Added<ShelfModel>>,
) {
    for (entity, model) in &query {
        commands
            .entity(entity)
            .insert((
                Visibility::Visible,
                Sprite::from_color(
                    Color::srgb(0.8, 0.6, 0.6),
                    Vec2::new(2.0 * model.half_x, 1.0),
                ),
                RigidBody::Fixed,
                Collider::cuboid(model.half_x, 0.5),
                OneWayPlatform,
                ActiveHooks::FILTER_CONTACT_PAIRS,
                ZIndex(-1),
            ))
            .remove::<ShelfModel>();
    }
}

use super::*;
use crate::constants::Scalar;

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct BarrierModel {
    pub x_length: Scalar,
    pub y_length: Scalar,
}

pub fn setup_barrier_model(
    mut commands: Commands,
    query: Query<(Entity, &BarrierModel), Added<BarrierModel>>,
) {
    for (entity, model) in &query {
        let barrier = commands
            .spawn((
                Methexis(entity),
                Sprite::from_color(
                    bevy::color::palettes::basic::GRAY,
                    Vec2::new(model.x_length, model.y_length),
                ),
                RigidBody::Fixed,
                Collider::cuboid(model.x_length * 0.5, model.y_length * 0.5),
            ))
            .id();
        commands
            .entity(entity)
            .clone_with(barrier, |builder| {
                builder.deny_all().allow::<Transform>();
            })
            .remove::<BarrierModel>();
    }
}

use super::*;
use crate::constants::Scalar;

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct BarrierModel {
    pub half_x: Scalar,
    pub half_y: Scalar,
    pub color: (f32, f32, f32, f32),
}

pub fn spawn_barrier(commands: &mut Commands, entity: Entity, model: &BarrierModel) {
    let (red, green, blue, alpha) = model.color;
    let barrier = commands
        .spawn((
            Methexis(entity),
            Sprite::from_color(
                Color::srgba(red, green, blue, alpha),
                Vec2::new(model.half_x * 2.0, model.half_y * 2.0),
            ),
            RigidBody::Fixed,
            Collider::cuboid(model.half_x, model.half_y),
        ))
        .id();
    commands.entity(entity).clone_with(barrier, |builder| {
        builder.deny_all().allow::<Transform>();
    });
}

pub fn setup_barrier_model(
    mut commands: Commands,
    query: Query<(Entity, &BarrierModel), Added<BarrierModel>>,
) {
    for (entity, model) in &query {
        spawn_barrier(&mut commands, entity, model);
        commands.entity(entity).remove::<BarrierModel>();
    }
}

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct PlatformRoomModel {
    pub h: Scalar,
    pub v: Scalar,
    pub r: Scalar,
}

pub fn setup_platform_room_model(
    mut commands: Commands,
    query: Query<(Entity, &PlatformRoomModel), Added<PlatformRoomModel>>,
) {
    for (entity, model) in &query {
        let transparent = (0.0, 0.0, 0.0, 0.0);
        let silver = (0.75, 0.75, 0.75, 1.0);
        commands
            .entity(entity)
            .insert(children![
                (
                    BarrierModel {
                        half_x: model.h,
                        half_y: model.r,
                        color: silver,
                    },
                    Transform::from_xyz(0.0, -model.v, 0.0),
                ),
                (
                    BarrierModel {
                        half_x: model.h,
                        half_y: model.r,
                        color: transparent,
                    },
                    Transform::from_xyz(0.0, model.v, 0.0),
                ),
                (
                    BarrierModel {
                        half_x: model.r,
                        half_y: model.v - model.r * 2.0,
                        color: transparent,
                    },
                    Transform::from_xyz(model.h, 0.0, 0.0),
                ),
                (
                    BarrierModel {
                        half_x: model.r,
                        half_y: model.v - model.r * 2.0,
                        color: transparent,
                    },
                    Transform::from_xyz(-model.h, 0.0, 0.0),
                )
            ])
            .remove::<PlatformRoomModel>();
    }
}

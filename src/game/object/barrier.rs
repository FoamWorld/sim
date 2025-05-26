use super::*;
use crate::constants::Scalar;

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct BarrierModel {
    pub x_half: Scalar,
    pub y_half: Scalar,
    pub color: (f32, f32, f32, f32),
}

pub fn spawn_barrier(commands: &mut Commands, entity: Entity, model: &BarrierModel) {
    let (red, green, blue, alpha) = model.color;
    let barrier = commands
        .spawn((
            Methexis(entity),
            Sprite::from_color(
                Color::srgba(red, green, blue, alpha),
                Vec2::new(model.x_half * 2.0, model.y_half * 2.0),
            ),
            RigidBody::Fixed,
            Collider::cuboid(model.x_half, model.y_half),
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
                        x_half: model.h,
                        y_half: model.r,
                        color: silver,
                    },
                    Transform::from_xyz(0.0, -model.v, 0.0),
                ),
                (
                    BarrierModel {
                        x_half: model.h,
                        y_half: model.r,
                        color: transparent,
                    },
                    Transform::from_xyz(0.0, model.v, 0.0),
                ),
                (
                    BarrierModel {
                        x_half: model.r,
                        y_half: model.v - model.r * 2.0,
                        color: transparent,
                    },
                    Transform::from_xyz(model.h, 0.0, 0.0),
                ),
                (
                    BarrierModel {
                        x_half: model.r,
                        y_half: model.v - model.r * 2.0,
                        color: transparent,
                    },
                    Transform::from_xyz(-model.h, 0.0, 0.0),
                )
            ])
            .remove::<PlatformRoomModel>();
    }
}

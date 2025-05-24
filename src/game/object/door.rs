use super::*;
use crate::physics::collision::BackgroundLevel;

pub enum DoorStatus {
    Open,
    Closed,
    Locked,
    Invalid,
}

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct DoorModel;

pub fn setup_door_model(
    mut commands: Commands,
    query: Query<(Entity, &DoorModel), Added<DoorModel>>,
) {
    for (entity, _) in &query {
        let door = commands
            .spawn((
                Methexis(entity),
                RigidBody::Fixed,
                RigidBodyDisabled,
                BackgroundLevel,
                Collider::cuboid(16.0, 32.0),
                ColliderMassProperties::Mass(10.0),
            ))
            .id();
        commands
            .entity(entity)
            .insert(IconImage {
                sheet: "door".to_string(),
                index: Some(0),
            })
            .clone_with(door, |builder| {
                builder.deny_all().allow::<Transform>();
            })
            .remove::<DoorModel>();
    }
}

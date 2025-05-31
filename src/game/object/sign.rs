use super::*;

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct SignModel(usize);

pub fn setup_door_model(
    mut commands: Commands,
    query: Query<(Entity, &SignModel), Added<SignModel>>,
) {
    for (entity, model) in &query {
        let sign = commands
            .spawn((
                Methexis(entity),
                RigidBody::Fixed,
                RigidBodyDisabled,
                Collider::cuboid(28.0, 26.0),
                ColliderMassProperties::Mass(40.0),
            ))
            .id();
        commands
            .entity(entity)
            .insert(IconImage {
                sheet: "sign".to_string(),
                index: Some(model.0),
            })
            .clone_with(sign, |builder| {
                builder.deny_all().allow::<Transform>();
            })
            .remove::<SignModel>();
    }
}

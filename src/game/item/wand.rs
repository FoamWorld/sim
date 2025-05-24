use super::*;
use crate::assets::RpgTextures;
use avian2d::prelude::*;

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct WandModel {
    pub mode: usize,
}

pub fn setup_wand_model(mut commands: Commands, query: Query<(Entity, &WandModel), Added<WandModel>>) {
    for (entity, model) in &query {
        commands
            .entity(entity)
            .insert((
                ActivateCommand::new(
                    |commands: &mut Commands,
                     world: &World,
                     entity: Entity,
                     source: Vec2,
                     target: Vec2| {
                        if let Some(mode) = world.entity(entity).get::<Mode>() {
                            commands.queue(LaunchMagic {
                                id: mode.0,
                                source,
                                target,
                            });
                        }
                    },
                ),
                ModifyCommand::new(|commands: &mut Commands, entity: Entity| {
                    commands
                        .entity(entity)
                        .entry::<Mode>()
                        .and_modify(|mut mode| {
                            mode.0 = if mode.0 == 3 { 0 } else { mode.0 + 1 };
                        });
                }),
                HoldsConfig::Wand,
                IconImage {
                    sheet: "items".to_string(),
                    index: Some(0),
                },
                Mode(model.mode),
                PhysicsConfig { mass: 0.5 },
            ))
            .remove::<WandModel>();
    }
}

struct LaunchMagic {
    id: usize,
    source: Vec2,
    target: Vec2,
}

impl Command for LaunchMagic {
    fn apply(self, world: &mut World) {
        let source = self.source;
        let ray = self.target - source;
        let unit = ray / ray.length();

        let mut commands = world.commands();
        let mut ammo = commands.spawn((
            Transform::from_xyz(source.x + unit.x * 24.0, source.y + unit.y * 24.0, 0.0),
            RigidBody::Dynamic,
            Collider::circle(6.0),
            LockedAxes::ROTATION_LOCKED,
            LinearVelocity(unit * 40.0),
            Mass(1.0),
            GravityScale(0.01),
            crate::game::mob::health::Health::fragile(),
        ));

        ammo.queue(move |mut entity: EntityWorldMut<'_>| {
            let sprite = {
                let rpg_folder = entity.world().resource::<RpgTextures>();
                Sprite::from_atlas_image(
                    rpg_folder.get_image_handle("spells"),
                    rpg_folder.get_texture_atlas("spells", self.id),
                )
            };
            entity.insert(sprite);
        });
    }
}

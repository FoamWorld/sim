use super::*;
use crate::{assets::RpgTextures, game::summon::elements::*};

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct WandModel {
    pub mode: usize,
}

pub fn setup_wand_model(
    mut commands: Commands,
    query: Query<(Entity, &WandModel), Added<WandModel>>,
) {
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
                            mode.0 = if mode.0 == 1 { 0 } else { 1 };
                        });
                }),
                HoldsConfig::Wand,
                IconImage {
                    sheet: "wands".to_string(),
                    index: Some(0),
                },
                Mode(model.mode),
                PhysicsConfig {
                    mass: 0.5,
                    shape: Vec2 { x: 7.0, y: 1.0 },
                },
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

        let sprite = {
            let rpg_folder = world.resource::<RpgTextures>();
            Sprite::from_atlas_image(
                rpg_folder.get_image_handle("spells"),
                rpg_folder.get_texture_atlas("spells", self.id),
            )
        };

        let mut commands = world.commands();
        let mut ammo = commands.spawn(sprite);

        if self.id == 0 {
            ammo.insert((
                Transform::from_xyz(source.x + unit.x * 24.0, source.y + unit.y * 24.0, 0.0),
                Velocity::linear(unit * 120.0),
            ));
            insert_middle_ball(&mut ammo);
            insert_fire(&mut ammo);
        } else if self.id == 1 {
            ammo.insert((
                Transform::from_xyz(source.x + unit.x * 24.0, source.y + unit.y * 24.0, 0.0)
                    .with_rotation(Quat::from_rotation_z(ops::atan2(unit.y, unit.x))),
                Velocity::linear(unit * 1200.0),
            ));
            insert_middle_arrow(&mut ammo);
            insert_ice(&mut ammo);
        }
    }
}

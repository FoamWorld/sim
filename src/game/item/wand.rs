use super::*;
use crate::{assets::RpgTextures, game::object::Object};
use avian2d::prelude::*;
use bevy::sprite::Anchor;

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component, Object, Item)]
#[type_path = "sim::item"]
pub struct Wand {
    pub mode: usize,
}

impl Object for Wand {
    fn texture_info(&self) -> Option<(&str, Option<usize>)> {
        Some(("items", Some(0)))
    }

    fn texture_anchor(&self) -> Anchor {
        Anchor::Custom(Vec2::new(-0.4, -0.4))
    }

    fn add_active_components(&self, commands: &mut EntityCommands) {
        commands.insert(crate::physics::camera::RotateWithMouse(
            Quat::from_rotation_z(-std::f32::consts::PI * 0.25),
        ));
    }
}

struct LaunchMagic {
    id: usize,
    source: Vec2,
    target: Option<Vec2>,
}

impl Command for LaunchMagic {
    fn apply(self, world: &mut World) {
        let source = self.source;
        let ray = if let Some(tar) = self.target {
            tar - source
        } else {
            Vec2::new(1.0, 0.0)
        };
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

impl Item for Wand {
    fn activate(&self, commands: &mut Commands, _: Entity, source: Vec2, target: Option<Vec2>) {
        commands.queue(LaunchMagic {
            id: self.mode,
            source,
            target,
        });
    }

    fn modify(&self, commands: &mut Commands, entity: Entity) {
        let mode = if self.mode == 3 { 0 } else { self.mode + 1 };
        commands
            .entity(entity)
            .entry::<Wand>()
            .and_modify(move |mut wand| {
                wand.mode = mode;
            });
    }
}

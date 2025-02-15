use super::*;
use crate::{assets::RpgTextures, game::object::Object};
use avian2d::prelude::*;

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Object, Item)]
pub struct DebugWand {
    pub mode: usize,
}

impl Object for DebugWand {
    fn add_visual_components(&self, commands: &mut EntityCommands, rpg_folder: &Res<RpgTextures>) {
        commands.insert(Sprite {
            image: rpg_folder.get_image_handle("items"),
            texture_atlas: Some(rpg_folder.get_texture_atlas("items", 0)),
            anchor: bevy::sprite::Anchor::Custom(Vec2::new(-0.4, -0.4)),
            ..default()
        });
    }
    fn add_extra_components(&self, commands: &mut EntityCommands) {
        commands.insert(crate::physics::camera::RotateWithMouse(
            Quat::from_rotation_z(-std::f32::consts::PI * 0.25),
        ));
    }
}

impl Item for DebugWand {
    fn item_use(
        &self,
        commands: &mut Commands,
        _: Entity,
        hold_point: Vec2,
        coords: Option<Vec2>,
        rpg_folder: Res<RpgTextures>,
    ) {
        let ray = coords.unwrap() - hold_point;
        let unit = ray / ray.length();

        commands.spawn((
            Sprite::from_atlas_image(
                rpg_folder.get_image_handle("spells"),
                rpg_folder.get_texture_atlas("spells", self.mode),
            ),
            Transform::from_xyz(
                hold_point.x + unit.x * 24.0,
                hold_point.y + unit.y * 24.0,
                0.0,
            ),
            RigidBody::Dynamic,
            Collider::circle(6.0),
            LockedAxes::ROTATION_LOCKED,
            LinearVelocity(unit * 40.0),
            crate::game::health::Health::fragile(),
        ));
    }
    fn check_can_consume(&self) -> bool {
        true
    }
    fn item_consume(&mut self) {
        self.mode += 1;
        if self.mode >= 4 {
            self.mode = 0;
        }
    }
}

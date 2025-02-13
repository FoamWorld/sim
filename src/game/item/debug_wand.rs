use crate::{assets::RpgTextures, game::object::Object};
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
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

impl super::Item for DebugWand {
    fn feed_to_storage(&self) {}
    fn item_use(
        &mut self,
        commands: &mut Commands,
        hold_point: Vec2,
        coords: Res<crate::physics::camera::CursorCoords>,
        rpg_folder: Res<RpgTextures>,
    ) {
        let ray = coords.0.unwrap() - hold_point;
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
    fn item_consume(&self) {}
}

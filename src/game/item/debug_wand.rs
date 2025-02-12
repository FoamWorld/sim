use crate::{assets::RpgTextures, game::object::Object};
use bevy::prelude::*;

#[derive(Component)]
pub struct DebugWand {
    pub mode: usize,
}

impl Object for DebugWand {
    fn add_sprite_components(&self, commands: &mut EntityCommands, rpg_folder: &Res<RpgTextures>) {
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
	fn feed_to_storage() {
	}
}

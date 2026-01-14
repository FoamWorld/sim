use super::*;
use crate::{assets::MyTextures, constants::Scalar};
use bevy::sprite;

#[derive(Reflect, Clone)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct ClothModel {
    pub h: Scalar,
    pub v: Scalar,
    pub bg: String,
}

impl Component for ClothModel {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, context| {
            let (h, v, bg) = {
                let model = world.entity(context.entity).get::<ClothModel>().unwrap();
                (model.h, model.v, model.bg.as_str())
            };
            let sprite = if bg != "" {
                let mut spr =
                    sprite::Sprite::from_image(world.resource::<MyTextures>().get_image_handle(bg));
                spr.custom_size = Some(Vec2::new(2.0 * h, 2.0 * v));
                spr.image_mode = sprite::SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: true,
                    stretch_value: 1.0,
                };
                spr
            } else {
                sprite::Sprite::from_color(Color::srgb(0.7, 0.7, 0.7), Vec2::new(2.0 * h, 2.0 * v))
            };
            let mut binding = world.commands();
            let mut commands = binding.entity(context.entity);
            commands.insert(children![(Transform::from_xyz(0.0, 0.0, -2.0), sprite,)]);
            commands.remove::<ClothModel>();
        })
    }
}

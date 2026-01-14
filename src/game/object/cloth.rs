use super::*;
use crate::{assets::MyTextures, constants::Scalar};
use bevy::sprite;

#[derive(Reflect, Clone)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct ClothModel {
    pub x: Scalar,
    pub y: Scalar,
    pub bg: String,
}

impl Component for ClothModel {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, context| {
            let (x, y, bg) = {
                let model = world.entity(context.entity).get::<ClothModel>().unwrap();
                (model.x, model.y, model.bg.as_str())
            };
            let sprite = if bg != "" {
                let mut spr =
                    sprite::Sprite::from_image(world.resource::<MyTextures>().get_image_handle(bg));
                spr.custom_size = Some(Vec2::new(x, y));
                spr.image_mode = sprite::SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: true,
                    stretch_value: 1.0,
                };
                spr
            } else {
                sprite::Sprite::from_color(Color::srgb(0.7, 0.7, 0.7), Vec2::new(x, y))
            };
            let mut binding = world.commands();
            let mut commands = binding.entity(context.entity);
            commands.insert(children![(Transform::from_xyz(0.0, 0.0, -2.0), sprite,)]);
            commands.remove::<ClothModel>();
        })
    }
}

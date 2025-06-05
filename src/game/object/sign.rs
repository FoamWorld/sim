use super::*;
use crate::assets::MyTextures;

#[derive(Reflect, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct HintModel(pub usize);

impl Component for HintModel {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, context| {
            let id = world.entity(context.entity).get::<HintModel>().unwrap().0;
            let sprite = {
                let textures = world.resource::<MyTextures>();
                Sprite::from_atlas_image(
                    textures.get_image_handle("hints"),
                    textures.get_texture_atlas("hints", id),
                )
            };

            let mut binding = world.commands();
            let mut commands = binding.entity(context.entity);
            commands.insert(sprite);
            commands.remove::<HintModel>();
        })
    }
}

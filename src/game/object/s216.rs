use super::*;
use crate::assets::MyTextures;

#[derive(Reflect, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct S216Model;

impl Component for S216Model {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, context| {
            let sprite = {
                let textures = world.resource::<MyTextures>();
                Sprite::from_image(textures.get_image_handle("scp_216"))
            };

            let mut binding = world.commands();
            let mut commands = binding.entity(context.entity);
            commands.insert(sprite);
            commands.remove::<S216Model>();
        })
    }
}

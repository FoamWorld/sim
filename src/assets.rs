use crate::state::AppState;
use bevy::{asset::LoadedFolder, image::ImageSampler, prelude::*};
use std::collections::HashMap;

#[derive(Resource, Clone)]
pub struct RpgTextures {
    pub folder: Handle<LoadedFolder>,
    pub dictionary: HashMap<String, Option<(u32, u32, u32)>>,
    pub named_images: HashMap<String, Handle<Image>>,
    pub named_atlases_layout: HashMap<String, Handle<TextureAtlasLayout>>,
}

impl RpgTextures {
    fn new(
        folder: Handle<LoadedFolder>,
        dictionary: HashMap<String, Option<(u32, u32, u32)>>,
    ) -> Self {
        Self {
            folder,
            dictionary,
            named_images: HashMap::<_, _>::new(),
            named_atlases_layout: HashMap::<_, _>::new(),
        }
    }
    fn process(
        &mut self,
        loaded_folders: Res<Assets<LoadedFolder>>,
        mut textures: ResMut<Assets<Image>>,
        mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let loaded_folder = loaded_folders.get(&self.folder).unwrap();
        for handle in loaded_folder.handles.iter() {
            // Gets file name.
            let asset_path = handle.path().unwrap();
            let file_name = asset_path.path().file_stem().unwrap();
            let string = std::ffi::OsString::from(file_name).into_string().unwrap();

            // Gets image handle.
            let id = handle.id().typed_unchecked::<Image>();
            let Some(texture) = textures.get_strong_handle(id) else {
                warn!("{asset_path} did not resolve to an `Image` asset.");
                continue;
            };

            // Config sampler.
            let image = textures.get_mut(id).unwrap();
            image.sampler = ImageSampler::nearest();

            // Inserts.
            self.named_images.insert(string.clone(), texture);
            if let Some(tuple) = self.dictionary.get(&string).unwrap() {
                let (size, columns, rows) = tuple;
                let texture_atlas_layout =
                    TextureAtlasLayout::from_grid(UVec2::splat(*size), *columns, *rows, None, None);
                self.named_atlases_layout
                    .insert(string.clone(), texture_atlases.add(texture_atlas_layout));
            } else {
                continue;
            }
        }
    }
    pub fn get_image_handle(&self, name: &str) -> Handle<Image> {
        let handle = self.named_images.get(name.into()).unwrap();
        handle.clone_weak()
    }
    pub fn get_texture_atlas(&self, name: &str, index: usize) -> TextureAtlas {
        let handle = self.named_atlases_layout.get(name.into()).unwrap();
        TextureAtlas {
            layout: handle.clone_weak(),
            index,
        }
    }
}

pub fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let list: Vec<(String, Option<(u32, u32, u32)>)> = vec![
        // todo: add "notexture" fallback
        ("character".into(), None),
        ("sign".into(), Some((32, 3, 5))),
        ("spells".into(), Some((16, 4, 1))),
        ("items".into(), Some((16, 3, 1))),
        ("hint1".into(), None),
    ];
    commands.insert_resource(RpgTextures::new(
        asset_server.load_folder("textures"),
        list.into_iter().collect(),
    ));
}

pub fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    mut texture_folder: ResMut<RpgTextures>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    textures: ResMut<Assets<Image>>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    for event in events.read() {
        if event.is_loaded_with_dependencies(&texture_folder.folder) {
            texture_folder.process(loaded_folders, textures, texture_atlases);
            next_state.set(AppState::Menu);
            return;
        }
    }
}

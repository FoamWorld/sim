use crate::state::AppState;
use bevy::{asset::LoadedFolder, image::ImageSampler, prelude::*};
use std::collections::HashMap;

#[derive(serde::Deserialize, Clone)]
pub struct ModeConfig {
    pub title: String,
    pub entrance: String,
    pub level: u16,
}

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct ModesConfig {
    pub level: u16,
    pub modes: Vec<ModeConfig>,
}

#[derive(Resource)]
pub struct ModesConfigHandle(pub Handle<ModesConfig>);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let modes = ModesConfigHandle(asset_server.load("config/default.modes.toml"));
    commands.insert_resource(modes);
}

#[derive(Resource, Clone)]
pub struct MyTextures {
    pub folder: Handle<LoadedFolder>,
    pub dictionary: HashMap<String, (u32, u32, u32, u32)>,
    pub named_images: HashMap<String, Handle<Image>>,
    pub named_atlases_layout: HashMap<String, Handle<TextureAtlasLayout>>,
}

impl MyTextures {
    fn new(
        folder: Handle<LoadedFolder>,
        dictionary: HashMap<String, (u32, u32, u32, u32)>,
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
            let string = file_name.to_str().unwrap().to_string();

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
            if let Some(tuple) = self.dictionary.get(&string) {
                let (width, length, columns, rows) = tuple;
                let texture_atlas_layout = TextureAtlasLayout::from_grid(
                    UVec2::new(*width, *length),
                    *columns,
                    *rows,
                    None,
                    None,
                );
                self.named_atlases_layout
                    .insert(string.clone(), texture_atlases.add(texture_atlas_layout));
            } else {
                continue;
            }
        }
    }

    pub fn get_image_handle(&self, name: &str) -> Handle<Image> {
        let handle = self
            .named_images
            .get(&String::from(name))
            .unwrap_or(self.named_images.get("fallback").unwrap());
        handle.clone()
    }

    pub fn get_texture_atlas(&self, name: &str, index: usize) -> TextureAtlas {
        if let Some(handle) = self.named_atlases_layout.get(&String::from(name)) {
            TextureAtlas {
                layout: handle.clone(),
                index,
            }
        } else {
            TextureAtlas {
                layout: self.named_atlases_layout.get("fallback").unwrap().clone(),
                index: 0,
            }
        }
    }
}

pub fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let list: Vec<(String, (u32, u32, u32, u32))> = vec![
        ("fallback".into(), (12, 12, 1, 1)),
        ("door".into(), (16, 32, 2, 2)),
        ("hints".into(), (13, 13, 6, 5)),
        ("wands".into(), (15, 7, 3, 1)),
        ("sign".into(), (32, 32, 3, 5)),
        ("spells".into(), (15, 15, 2, 1)),
    ];
    commands.insert_resource(MyTextures::new(
        asset_server.load_folder("textures"),
        list.into_iter().collect(),
    ));
}

pub fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    mut texture_folder: ResMut<MyTextures>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    textures: ResMut<Assets<Image>>,
    mut events: MessageReader<AssetEvent<LoadedFolder>>,
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

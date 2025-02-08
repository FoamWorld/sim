use crate::state::AppState;
use bevy::{asset::LoadedFolder, prelude::*};

#[derive(Resource)]
pub struct RpgTextureFolder(pub Handle<LoadedFolder>);

pub fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(RpgTextureFolder(asset_server.load_folder("textures")));
}

pub fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    texture_folder: Res<RpgTextureFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    for event in events.read() {
        if event.is_loaded_with_dependencies(&texture_folder.0) {
            next_state.set(AppState::Menu);
        }
    }
}

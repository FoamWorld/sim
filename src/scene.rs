use crate::{
    character::*,
    game::{health::*, item::*, object::*},
    state::*,
};
use bevy::{prelude::*, scene::*, tasks::IoTaskPool};
use std::{fs::File, io::Write};

#[derive(Resource)]
pub struct StorageSlotInfo(pub String);

pub struct RegisteryPlugin;

impl Plugin for RegisteryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StorageSlotInfo("slot1".to_string()));
        app.register_type::<Actor>()
            .register_type::<Health>()
            .register_type::<ItemStorage>()
            // objects
            .register_type::<IsObject>()
            .register_type::<Barrier>()
            .register_type::<NonUnique>()
            // items
            .register_type::<IsItem>()
            .register_type::<sign::Sign>()
            .register_type::<debug_wand::DebugWand>();
    }
}

pub fn load_scene_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<StorageState>>,
) {
    next_state.set(StorageState::Loading);
    // todo: add transform
    commands
        .spawn(DynamicSceneRoot(asset_server.load("scenes/debug.scn.ron")))
        .observe(
            |_: Trigger<SceneInstanceReady>,
             mut change_state: ResMut<NextState<StorageState>>,
             mut change_state2: ResMut<NextState<LoadingState>>| {
                change_state.set(StorageState::None);
                change_state2.set(LoadingState::Processing);
            },
        );
}

pub fn setup_game(world: &World, mut commands: Commands, query: Query<(Entity, &IsObject)>) {
    let type_registry = world.get_resource::<AppTypeRegistry>().unwrap();
    for (entity, marker) in query.iter() {
        let mut ec = commands.entity(entity);
        marker.add_components(
            world,
            &mut ec,
            entity,
            type_registry,
            AdditionConfig::IN_SCENE,
        );
    }
}

pub fn save_scene_system(world: &mut World) {
    let scene = {
        let mut query = world.query_filtered::<Entity, With<IsObject>>();
        let scene_builder = DynamicSceneBuilder::from_world(&world)
            .deny_all()
            .allow_component::<Transform>()
            .allow_component::<Health>()
            // objects
            .allow_component::<IsObject>()
            .allow_component::<Barrier>()
            .allow_component::<NonUnique>()
            // items
            .allow_component::<IsItem>()
            .allow_component::<ItemStorage>()
            .allow_component::<debug_wand::DebugWand>();
        scene_builder.extract_entities(query.iter(&world)).build()
    };

    let type_registry = world.get_resource::<AppTypeRegistry>().unwrap();
    let binding = type_registry.read();
    let serialized_scene = scene.serialize(&binding).unwrap();

    let info = world.get_resource::<StorageSlotInfo>().unwrap();

    let dist = "saved/".to_string() + info.0.as_str() + "/scenes/1.scn.ron";

    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(dist)
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}

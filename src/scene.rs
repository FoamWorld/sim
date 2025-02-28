use crate::{
    character::*,
    game::{health::*, item::*, object::*},
};
use bevy::{prelude::*, tasks::IoTaskPool};
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
            .register_type::<IsObject>()
            .register_type::<Barrier>()
            .register_type::<NonUnique>()
            .register_type::<IsItem>()
            .register_type::<sign::Sign>()
            .register_type::<debug_wand::DebugWand>();
    }
}

#[allow(dead_code, reason = "used in feature devtools")]
pub fn save_scene_system(world: &mut World) {
    let type_registry = world.get_resource::<AppTypeRegistry>().unwrap();

    let scene = DynamicSceneBuilder::from_world(&world)
        .deny_all()
        .allow_component::<Transform>()
        .allow_component::<Health>()
        .allow_component::<IsObject>()
        .allow_component::<IsItem>()
        .allow_component::<ItemStorage>()
        .allow_component::<debug_wand::DebugWand>()
        .extract_entities(world.iter_entities().map(|entity| entity.id()))
        .build();

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

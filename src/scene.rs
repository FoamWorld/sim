use crate::{
    control::*,
    game::{health::*, item::*, object::*},
};
use bevy::{prelude::*, tasks::IoTaskPool};
use std::{fs::File, io::Write};

pub struct RegisteryPlugin;

impl Plugin for RegisteryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .register_type::<Barrier>()
            .register_type::<NonUnique>()
            .register_type::<debug_wand::DebugWand>();
    }
}

pub fn save_scene_system(world: &mut World) {
    let type_registry = world.get_resource::<AppTypeRegistry>().unwrap();

    let scene = DynamicSceneBuilder::from_world(&world)
        .deny_all()
        .allow_component::<Transform>()
        .allow_component::<Actor>()
        .allow_component::<Health>()
        .allow_component::<ItemStorage>()
        .allow_component::<debug_wand::DebugWand>()
        .extract_entities(world.iter_entities().map(|entity| entity.id()))
        .build();

    let binding = type_registry.read();
    let serialized_scene = scene.serialize(&binding).unwrap();

    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(String::from("saved/scenes/1.scn.ron"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}

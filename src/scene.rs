use crate::{
    character::*,
    game::{item::*, mob::*, object::*},
    physics::collision::*,
    state::*,
};
use bevy::{prelude::*, scene::*, tasks::IoTaskPool};
use std::{fs::File, io::Write};

#[derive(Resource)]
pub struct StorageSlotInfo(pub String);

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcessSet {
    Early,
    Middle,
    Late,
    TransitState,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProcessState {
    None,
    LoadScene,
    PostLoadScene,
    PreEnterGame,
    PreSaveScene,
}

pub struct RegisteryPlugin;

impl Plugin for RegisteryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ProcessState::None);

        app.configure_sets(
            FixedUpdate,
            (
                ProcessSet::Early,
                ProcessSet::Middle,
                ProcessSet::Late,
                ProcessSet::TransitState,
            )
                .chain()
                .run_if(in_state(GameState::Locked))
                .run_if(in_state(AppState::InGame)),
        );

        app.add_systems(
            OnEnter(AppState::InGame),
            load_scene_system.in_set(ProcessSet::Early),
        );

        app.add_systems(
            OnEnter(ProcessState::PostLoadScene),
            (
                process_loaded_scene,
                |mut next_state: ResMut<NextState<ProcessState>>| {
                    next_state.set(ProcessState::PreEnterGame);
                },
            )
                .chain()
                .in_set(ProcessSet::Early),
        );

        app.add_systems(
            OnEnter(ProcessState::PreEnterGame),
            (
                setup_character.in_set(ProcessSet::Middle),
                (|mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::Running);
                })
                .in_set(ProcessSet::TransitState),
            ),
        );

        app.add_systems(OnEnter(ProcessState::PreSaveScene), save_scene_system);

        app.insert_resource(StorageSlotInfo("slot1".to_string()));
        app
            // core
            // utils
            .register_type::<Actor>()
            .register_type::<ItemStorage>()
            .register_type::<Sign>()
            // objects
            .register_type::<IsObject>()
            .register_type::<Barrier>()
            .register_type::<NonUnique>()
            .register_type::<sign::SignStand>()
            .register_type::<door::Door>()
            // items
            .register_type::<IsItem>()
            .register_type::<debug_wand::DebugWand>()
            // mobs
            .register_type::<health::Health>();
    }
}

pub fn load_scene_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<ProcessState>>,
) {
    next_state.set(ProcessState::LoadScene);
    // todo: add transform
    commands
        .spawn(DynamicSceneRoot(asset_server.load("scenes/debug.scn.ron")))
        .observe(
            |_: Trigger<SceneInstanceReady>, mut next_state: ResMut<NextState<ProcessState>>| {
                next_state.set(ProcessState::PostLoadScene);
            },
        );
}

pub fn process_loaded_scene(
    world: &World,
    mut commands: Commands,
    query: Query<(Entity, &IsObject)>,
) {
    let type_registry = world.resource::<AppTypeRegistry>();
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
            // core
            .allow_component::<Transform>()
            // utils
            .allow_component::<Sign>()
            // objects
            .allow_component::<IsObject>()
            .allow_component::<Barrier>()
            .allow_component::<NonUnique>()
            .allow_component::<sign::SignStand>()
            .allow_component::<door::Door>()
            // items
            .allow_component::<IsItem>()
            .allow_component::<ItemStorage>()
            .allow_component::<debug_wand::DebugWand>()
            // mobs
            .allow_component::<health::Health>();
        scene_builder.extract_entities(query.iter(&world)).build()
    };

    let type_registry = world.resource::<AppTypeRegistry>();
    let binding = type_registry.read();
    let serialized_scene = scene.serialize(&binding).unwrap();

    let info = world.resource::<StorageSlotInfo>();

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

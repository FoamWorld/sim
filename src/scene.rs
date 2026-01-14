use crate::{
    character::*,
    game::{ecs::*, item::*, item_storage::*, mob::*, object::*},
    physics::{camera::*, collision::*, *},
    state::*,
};
use bevy::{prelude::*, scene::*, tasks::IoTaskPool};
use std::{fs::File, io::Write};

#[derive(Reflect, Resource, Default, PartialEq)]
pub enum PortalTarget {
    #[default]
    None,

    /// Current scene.
    Current,

    /// Jumps to an unique scene (determined by file path).
    Unique(String),

    /// Jumps to a scene (determined by file path) and generates a variant id.
    Clone(String),

    /// Jumps to a specific scene (determined by file path and variant id).
    Specific(String, u32),
}

#[derive(Reflect, Component)]
#[reflect(Component)]
#[type_path = "sim::scene"]
pub struct PortalAnchor(pub u32);

#[derive(Reflect, Component)]
#[reflect(Component)]
#[type_path = "sim::scene"]
pub struct Portal {
    on: bool,
    target: PortalTarget,
    position: PortalAnchor,
}

#[derive(Resource)]
pub struct GameSave {
    base_path: String,
    current_slot: Option<String>,
}

impl GameSave {
    fn new() -> Self {
        GameSave {
            base_path: "saved".to_string(),
            current_slot: Some("default".into()),
        }
    }

    fn initialize(&self) {
        let _ = std::fs::create_dir_all(self.base_path.clone());
    }

    fn get_scene_path(&self, scene_name: &str) -> String {
        self.base_path.clone()
            + "/"
            + self.current_slot.clone().unwrap().as_str()
            + "/scenes/"
            + scene_name
            + ".ron"
    }
}

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
        app.insert_resource(GameSave::new())
            .init_resource::<PortalTarget>()
            .init_resource::<ActorStatus>();

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
            (|mut next_state: ResMut<NextState<ProcessState>>| {
                next_state.set(ProcessState::PreEnterGame);
            },)
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

        app
            // resource
            .register_type::<SceneBox>()
            .register_type::<CameraMoveConfig>()
            // model
            .register_type::<Position>()
            .register_type::<barrier::BarrierModel>()
            .register_type::<barrier::WallModel>()
            .register_type::<barrier::GroundModel>()
            .register_type::<cloth::ClothModel>()
            .register_type::<door::SideDoorModel>()
            .register_type::<platform::ShelfModel>()
            .register_type::<s216::S216Model>()
            .register_type::<wand::WandModel>()
            .register_type::<sign::HintModel>()
            // utils
            .register_type::<Methexis>()
            .register_type::<Eidos>()
            .register_type::<HoldsConfig>()
            .register_type::<IconImage>()
            .register_type::<Mode>()
            .register_type::<PhysicsConfig>()
            .register_type::<Actor>()
            .register_type::<ItemStorage>()
            .register_type::<Sign>()
            .register_type::<dummy::DamageSensor>()
            // mobs
            .register_type::<health::Health>();
    }
}

pub fn load_scene_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    target: Res<PortalTarget>,
    mut next_state: ResMut<NextState<ProcessState>>,
) {
    next_state.set(ProcessState::LoadScene);
    let name = if let PortalTarget::Unique(string) = target.into_inner() {
        string
    } else {
        warn!("Target not supported.");
        return;
    };

    commands
        .spawn((
            DynamicSceneRoot(asset_server.load(name.to_owned() + ".ron")),
            Visibility::Visible,
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .observe(
            |_: On<SceneInstanceReady>, mut next_state: ResMut<NextState<ProcessState>>| {
                next_state.set(ProcessState::PostLoadScene);
            },
        );
}

pub fn save_scene_system(world: &mut World) {
    let scene = {
        let mut query = world.query_filtered::<Entity, Or<(With<Eidos>, With<Methexis>)>>();
        let scene_builder = DynamicSceneBuilder::from_world(&world)
            .deny_all()
            // core
            .allow_component::<Transform>()
            // utils
            .allow_component::<ChildOf>()
            .allow_component::<Children>()
            .allow_component::<Methexis>()
            .allow_component::<Eidos>()
            .allow_component::<HoldsConfig>()
            .allow_component::<IconImage>()
            .allow_component::<Mode>()
            .allow_component::<PhysicsConfig>()
            .allow_component::<Sign>()
            .allow_component::<ItemStorage>()
            // mobs
            .allow_component::<health::Health>();

        scene_builder
            .extract_resources()
            .extract_entities(query.iter(&world))
            .build()
    };

    let type_registry = world.resource::<AppTypeRegistry>();
    let binding = type_registry.read();
    let serialized_scene = scene.serialize(&binding).unwrap();

    let info = world.resource::<GameSave>();
    info.initialize();

    let dist = info.get_scene_path("debug");

    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(dist)
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}

use crate::{
    assets::RpgTextures,
    constants::*,
    control::*,
    game::{
        item::{debug_wand::DebugWand, *},
        object::*,
        Inventory,
    },
};
use avian2d::prelude::*;
use bevy::prelude::*;
use std::any::Any;

#[derive(Component)]
pub struct IsActive;

#[derive(Reflect)]
pub enum ActorFacing {
    Left,
    Right,
}

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Actor(pub ActorFacing);

impl Actor {
    pub fn set_facing(&mut self, facing: ActorFacing) {
        self.0 = facing;
    }
}

pub fn setup_character(
    mut commands: Commands,
    rpg_folder: Res<RpgTextures>,
    mut inventory: ResMut<Inventory>,
) {
    let launcher = {
        let debug_wand = DebugWand { mode: 3 };
        commands
            .spawn((debug_wand, IsObject(debug_wand.type_id()), IsItem))
            .id()
    };

    let mut storage = ItemStorage::with_capacity(4);
    storage.force_give(0, launcher);
    inventory.size = 4;
    inventory.bind = Some(commands.spawn(storage).id());

    commands.spawn((
        Sprite {
            image: rpg_folder.get_image_handle("character"),
            custom_size: Some(Vec2::new(CHARACTER_X_LENGTH, CHARACTER_Y_LENGTH)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, CHARACTER_LAYER),
        RigidBody::Dynamic,
        Collider::rectangle(CHARACTER_X_LENGTH, CHARACTER_Y_LENGTH),
        LockedAxes::ROTATION_LOCKED,
        Mass(70.0),
        MovementSpeed(100.0),
        Actor(ActorFacing::Right),
    ));
}

#[derive(Component)]
pub struct UiGrid(pub usize);

pub fn setup_inventory(mut commands: Commands, world: &World, inventory: Res<Inventory>) {
    let type_registry = world.resource::<AppTypeRegistry>();
    let storage = world
        .entity(inventory.bind.unwrap())
        .get::<ItemStorage>()
        .unwrap();
    let size = inventory.size;
    let mut ui = commands.spawn((Node {
        position_type: PositionType::Absolute,
        height: Val::Px(GRID_SIZE),
        bottom: Val::Px(4.0),
        justify_self: JustifySelf::Center,
        justify_items: JustifyItems::Center,
        flex_direction: FlexDirection::Row,
        ..default()
    },));
    ui.with_children(|builder| {
        for ind in 0..size {
            let mut ec = builder.spawn((
                Node {
                    width: Val::Px(GRID_SIZE),
                    height: Val::Px(GRID_SIZE),
                    margin: UiRect::horizontal(Val::Px(4.0)),
                    ..default()
                },
                Outline::new(Val::Px(1.0), Val::ZERO, Color::BLACK),
                UiGrid(ind),
            ));
            if let Some(item) = storage.storage[ind] {
                let object = world.entity(item).get::<IsObject>().unwrap();
                object.add_components(world, &mut ec, item, type_registry, AdditionConfig::IN_GRID);
            } else {
                ec.insert(ImageNode::solid_color(Color::NONE));
            }
        }
    });
}

pub fn setup_attached_image(
    mut commands: Commands,
    world: &World,
    inventory: Res<Inventory>,
    actors: Query<Entity, With<Actor>>,
    q_obj: Query<&IsObject>,
) {
    let actor = actors.single();
    let inv = inventory.bind.unwrap();
    let storage = world.entity(inv).get::<ItemStorage>().unwrap();
    if let Some(item) = storage.get_index(inventory.selected) {
        commands
            .entity(actor)
            .with_children(|parent: &mut ChildBuilder<'_>| {
                setup_item_sprite(world, item, parent, q_obj);
            });
    };
}

pub fn setup_item_sprite(
    world: &World,
    item: Entity,
    parent: &mut ChildBuilder,
    q_obj: Query<&IsObject>,
) {
    let registry = world.resource::<AppTypeRegistry>();
    let it = q_obj.get(item).unwrap();
    let mut ec = parent.spawn((
        Transform::from_translation(CHARACTER_LEFT_HAND_OFFSET.extend(CHARACTER_HOLD_OFFSET)),
        IsActive,
    ));
    it.add_components(
        world,
        &mut ec,
        item,
        registry,
        AdditionConfig::CHARACTER_ATTACH,
    );
}

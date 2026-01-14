use super::*;
use crate::{
    assets::MyTextures,
    game::{item_control::*, summon::elements::*},
};

#[derive(Reflect, Clone, Copy)]
#[reflect(Component)]
#[type_path = "sim::model"]
pub struct WandModel {
    pub mode: usize,
}

impl Component for WandModel {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, context| {
            let id = {
                world
                    .entity(context.entity)
                    .get::<WandModel>()
                    .unwrap()
                    .mode
            };
            let mut binding = world.commands();
            let mut commands = binding.entity(context.entity);
            commands.insert((
                HoldsConfig::Wand,
                IconImage {
                    sheet: "wands".to_string(),
                    index: Some(id),
                },
                Mode(0),
                PhysicsConfig {
                    mass: 0.5,
                    shape: Vec2 { x: 7.0, y: 1.0 },
                },
            ));
            commands.observe(activate_wand);
            commands.observe(interact_wand);
            // TODO: Add another component so that observers can be added when loaded from save.
            commands.remove::<WandModel>();
        })
    }
}

fn activate_wand(activate: On<Activation>, mut commands: Commands, world: &World) {
    if let Some(mode) = world.entity(activate.entity).get::<Mode>() {
        commands.queue(LaunchMagic {
            id: mode.0,
            pointing: activate.pointing,
        });
    }
}

fn interact_wand(interact: On<Communication>, mut commands: Commands, world: &World) {
    if let Some(mode) = world.entity(interact.0).get::<Mode>() {
        commands
            .entity(interact.0)
            .insert(Mode(if mode.0 == 1 { 0 } else { 1 }));
    }
}

struct LaunchMagic {
    id: usize,
    pointing: ItemPointing,
}

impl Command for LaunchMagic {
    fn apply(self, world: &mut World) {
        let pointing = self.pointing;
        let source = pointing.source;
        let (s, c) = ops::sin_cos(pointing.rotation);
        let unit = Vec2::new(c, s);

        let sprite = {
            let textures = world.resource::<MyTextures>();
            Sprite::from_atlas_image(
                textures.get_image_handle("spells"),
                textures.get_texture_atlas("spells", self.id),
            )
        };

        let mut commands = world.commands();
        let mut ammo = commands.spawn(sprite);

        if self.id == 0 {
            ammo.insert((
                Transform::from_xyz(source.x + unit.x * 24.0, source.y + unit.y * 24.0, 0.0),
                Velocity::linear(unit * 60.0),
            ));
            insert_middle_ball(&mut ammo);
            insert_fire(&mut ammo);
        } else if self.id == 1 {
            ammo.insert((
                Transform::from_xyz(source.x + unit.x * 24.0, source.y + unit.y * 24.0, 0.0)
                    .with_rotation(Quat::from_rotation_z(ops::atan2(unit.y, unit.x))),
                Velocity::linear(unit * 1200.0),
            ));
            insert_middle_arrow(&mut ammo);
            insert_ice(&mut ammo);
        }
    }
}

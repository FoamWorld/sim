/// Introduces marker components that won't be saved.
use bevy::prelude::*;

/// Entities with this component won't be saved.
#[derive(Component)]
pub struct WontSave;

/// Root of ui fragment. Used for despawning.
#[derive(Component, PartialEq)]
pub enum  UiRoot {
	/// e.g. pause menu
	Menu,

	/// e.g. text with [`DespawnTimeout`]
	GameTemporary,

	// e.g. inventory
	GameRegular,

	// e.g. player opened gui
	GameDismissible,
}

/// Root of game objects.
#[derive(Component)]
pub struct ObjRoot;

/// Despawn timeout.
#[derive(Component)]
pub struct DespawnTimeout(pub Timer);

pub fn process_despawn_timeout(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: Query<(Entity, &mut DespawnTimeout)>,
) {
    for (entity, mut timer) in timers.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

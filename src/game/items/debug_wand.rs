use bevy::prelude::*;

#[derive(Component)]
pub struct DebugWand {
	mode: usize
}

impl Object for DebugWand {}

impl Item for DebugWand {}

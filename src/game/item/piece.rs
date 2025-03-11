use std::sync::Arc;
use super::*;

#[derive(Reflect, Component, Clone)]
#[reflect(Component, Object, Item)]
#[type_path = "sim::item"]
pub struct Piece {
	shape: Arc<Vec<Vec2>>,
}

impl Object for Piece {}

impl Item for Piece {
    fn activate(
        &self,
        commands: &mut Commands,
        entity: Entity,
        source: Vec2,
        target: Option<Vec2>,
    ) {
        todo!()
    }
}

use super::*;
use bevy_rapier2d::prelude::*;

#[derive(Resource, Default)]
pub struct HoverEntity {
    pub is_inside: bool,
    pub closest: Option<Entity>,
}

pub fn physics_hover_detection(
    read_rapier_context: ReadRapierContext,
    coords: Res<CursorCoords>,
    mut hover: ResMut<HoverEntity>,
) {
    let rapier_context = read_rapier_context.single().unwrap();
    if let Some(cursor_pos) = coords.0 {
        if let Some((entity, projection)) =
            rapier_context.project_point(cursor_pos, true, QueryFilter::only_dynamic())
        {
            hover.is_inside = projection.is_inside;
            hover.closest = Some(entity);
        };
    } else {
        hover.is_inside = false;
        hover.closest = None;
    }
}

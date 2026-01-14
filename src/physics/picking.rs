use super::*;
use crate::character::*;
use bevy_rapier2d::prelude::*;

#[derive(Resource, Default)]
pub struct HoverEntity {
    pub is_inside: bool,
    pub is_legal: bool,
    pub closest: Option<Entity>,
}

pub fn physics_hover_detection(
    read_rapier_context: ReadRapierContext,
    coords: Res<CursorCoords>,
    actors: Query<Entity, With<Actor>>,
    actor_pos: Res<ActorStatus>,
    mut hover: ResMut<HoverEntity>,
) {
    let rapier_context = read_rapier_context.single().unwrap();
    let actor = actors.single().unwrap();
    let filter = QueryFilter::only_dynamic().exclude_collider(actor);
    if let Some(cursor_pos) = coords.0 {
        if let Some((entity, projection)) =
            rapier_context.project_point(cursor_pos, 256.0, true, filter)
        {
            if projection.point.distance(cursor_pos) < 8.0 {
                hover.is_inside = projection.is_inside;
                hover.is_legal = true;
                hover.closest = Some(entity);
                return;
            }
        }
    }

    if let Some((entity, projection)) =
        rapier_context.project_point(actor_pos.center, 256.0, true, filter)
    {
        hover.is_inside = projection.is_inside;
        hover.is_legal = projection.point.distance(actor_pos.center) < 30.0;
        hover.closest = Some(entity);
        return;
    }
    hover.is_inside = false;
    hover.is_legal = false;
    hover.closest = None;
}

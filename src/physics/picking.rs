use super::*;
use crate::character::*;
use bevy_rapier2d::prelude::*;

#[derive(Resource, Default)]
pub struct HoverEntity {
    pub is_inside: bool,
    pub closest: Option<Entity>,
}

pub fn physics_hover_detection(
    read_rapier_context: ReadRapierContext,
    coords: Res<CursorCoords>,
    actors: Query<Entity, With<Actor>>,
    actor_pos: Res<ActorStatus>,
    mut hover: ResMut<HoverEntity>,
) {
    // Panics due to <https://github.com/dimforge/parry/commit/5a6c912779c36ff271aecd1b1a76e1a3df92f101#diff-4e777b7ebf29481b95fcffdcc0a1b21ffc1eebf829b3f70f2e4c5dbd002c4a96>.
    // TODO: Will be recovered later (while upgrading to parry 0.26).
    // Consider using bevy picking.
    /*
    let rapier_context = read_rapier_context.single().unwrap();
    let actor = actors.single().unwrap();
    let filter = QueryFilter::only_dynamic().exclude_collider(actor);
    if let Some(cursor_pos) = coords.0 {
        if let Some((entity, projection)) =
            rapier_context.project_point(cursor_pos, 8.0, true, filter)
        {
            hover.is_inside = projection.is_inside;
            hover.closest = Some(entity);
            return;
        }
    }

    if let Some((entity, projection)) =
        rapier_context.project_point(actor_pos.center, 30.0, true, filter)
    {
        hover.is_inside = projection.is_inside;
        hover.closest = Some(entity);
        return;
    }
    hover.is_inside = false;
    hover.closest = None;
    */
}

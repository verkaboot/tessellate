use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<TerrainCanvasDragged>();
}

#[derive(Event)]
pub struct TerrainCanvasDragged(pub Drag);

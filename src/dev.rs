use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{
    grid::{GridCoord, GridSettings},
    terrain::{TerrainList, TerrainType},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(WorldInspectorPlugin::new());

    app.register_type::<TerrainType>()
        .register_type::<TerrainList>()
        .register_type::<GridCoord>()
        .register_type::<GridSettings>();
}

use bevy::prelude::*;
use canvas::SIZE;

use crate::grid::{Grid, GridSettings};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GridSettings {
        cell_size: UVec2::new(SIZE.x, SIZE.y),
    })
    .insert_resource(TerrainBrush::default())
    .init_resource::<Grid>();
}

#[derive(Reflect, Component, Clone)]
#[reflect(Component)]
pub struct TerrainType {
    pub label: String,
    pub color: Color,
}

impl PartialEq for TerrainType {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct TerrainBrush {
    pub terrain_type: TerrainType,
}

impl Default for TerrainType {
    fn default() -> Self {
        TerrainType {
            label: "Default".to_owned(),
            color: Color::srgba(0.5, 0.7, 0.8, 0.7),
        }
    }
}

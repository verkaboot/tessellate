use bevy::prelude::*;
use canvas::SIZE;
use ui::widget::prelude::SelectList;
use ui_macros::SelectList;

use crate::grid::{Grid, GridSettings};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GridSettings {
        cell_size: UVec2::new(SIZE.x, SIZE.y),
    })
    .insert_resource(TerrainList::new(TerrainType::default()))
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

#[derive(Reflect, Resource, SelectList)]
#[reflect(Resource)]
pub struct TerrainList {
    selected: usize,
    list: Vec<TerrainType>,
}

impl Default for TerrainType {
    fn default() -> Self {
        TerrainType {
            label: "Default".to_owned(),
            color: Color::srgba(0.5, 0.7, 0.8, 0.7),
        }
    }
}

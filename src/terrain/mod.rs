use bevy::{ecs::system::IntoAdapterSystem, prelude::*, utils};
use canvas::{tool::ToolData, SIZE};
use input::trigger::OnDrag;
use ui::widget::prelude::SelectList;
use ui_macros::SelectList;

use error::Result;

use crate::grid::{Grid, GridCoord, GridSettings};

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

pub fn draw(
    tool_data: Res<ToolData>,
    grid_settings: Res<GridSettings>,
    mut grid: ResMut<Grid>,
    terrain_list: Res<TerrainList>,
    cells: Query<&TerrainType, With<GridCoord>>,
    mut commands: Commands,
) -> Result<()> {
    let coord = GridCoord::from_world_pos(tool_data.world_pos[0], *grid_settings);
    let cell_pos = coord.to_world_pos(*grid_settings);
    let terrain_type = terrain_list.get_selected();

    if let Some(&old_cell) = (*grid).get(&coord) {
        let old_terrain_type = cells.get(old_cell)?;
        if old_terrain_type == terrain_type {
            // Return early if the existing terrain type is the same
            return Ok(());
        } else {
            // Despawn the old cell
            commands.entity(old_cell).despawn_recursive();
        }
    }

    // Add the new cell
    let entity = commands
        .spawn((
            Name::new("TerrainSprite"),
            Sprite {
                color: terrain_type.color,
                custom_size: Some(SIZE.as_vec2()),
                anchor: bevy::sprite::Anchor::BottomLeft,
                ..default()
            },
            Transform::from_xyz(cell_pos.x, cell_pos.y, 0.0),
            coord,
            terrain_type.clone(),
        ))
        .id();

    // Add the new cell to the hashmap for easy grid lookup
    grid.insert(coord, entity);

    Ok(())
}

pub fn erase(
    _trigger: Trigger<OnDrag>,
    tool_data: Res<ToolData>,
    grid_settings: Res<GridSettings>,
    mut grid: ResMut<Grid>,
    mut commands: Commands,
) -> Result<()> {
    let coord = GridCoord::from_world_pos(tool_data.world_pos[0], *grid_settings);

    if let Some(cell_entity) = grid.remove(&coord) {
        commands.entity(cell_entity).despawn_recursive();
    }

    Ok(())
}

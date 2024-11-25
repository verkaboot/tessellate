use bevy::prelude::*;
use canvas::{tool::ToolData, SIZE};
use ui::interaction::OnDrag;

use crate::grid::{Grid, GridCoord, GridSettings};
use error::Result;

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
    label: String,
    color: Color,
}

impl PartialEq for TerrainType {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct TerrainBrush {
    terrain_type: TerrainType,
}

impl Default for TerrainType {
    fn default() -> Self {
        TerrainType {
            label: "Default".to_owned(),
            color: Color::srgba(0.5, 0.7, 0.8, 0.7),
        }
    }
}

pub fn draw_terrain(
    _trigger: Trigger<OnDrag>,
    tool_data: Res<ToolData>,
    grid_settings: Res<GridSettings>,
    mut grid: ResMut<Grid>,
    terrain_brush: Res<TerrainBrush>,
    cells: Query<&TerrainType, With<GridCoord>>,
    mut commands: Commands,
) -> Result<()> {
    let coord = GridCoord::from_world_pos(tool_data.world_pos[0], *grid_settings);
    let cell_pos = coord.to_world_pos(*grid_settings);

    if let Some(&old_cell) = (*grid).get(&coord) {
        println!("old_cell: {:?}", old_cell);
        let old_terrain_type = cells.get(old_cell)?;
        if *old_terrain_type == terrain_brush.terrain_type {
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
            SpriteBundle {
                sprite: Sprite {
                    color: terrain_brush.terrain_type.color,
                    custom_size: Some(SIZE.as_vec2()),
                    anchor: bevy::sprite::Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform::from_xyz(cell_pos.x, cell_pos.y, 0.0),
                ..default()
            },
            coord,
            terrain_brush.terrain_type.clone(),
        ))
        .id();

    // Add the new cell to the hashmap for easy grid lookup
    grid.insert(coord, entity);

    Ok(())
}

pub fn erase_terrain(
    _trigger: Trigger<OnDrag>,
    key: Res<ButtonInput<KeyCode>>,
    tool_data: Res<ToolData>,
    grid_settings: Res<GridSettings>,
    mut grid: ResMut<Grid>,
    cells: Query<&TerrainType, With<GridCoord>>,
    mut commands: Commands,
) -> Result<()> {
    let coord = GridCoord::from_world_pos(tool_data.world_pos[0], *grid_settings);
    let cell_pos = coord.to_world_pos(*grid_settings);

    if let Some(&old_cell) = (*grid).get(&coord) {
        println!("old_cell: {:?}", old_cell);
        let old_terrain_type = cells.get(old_cell)?;
        if *old_terrain_type == terrain_brush.terrain_type {
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
            SpriteBundle {
                sprite: Sprite {
                    color: terrain_brush.terrain_type.color,
                    custom_size: Some(SIZE.as_vec2()),
                    anchor: bevy::sprite::Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform::from_xyz(cell_pos.x, cell_pos.y, 0.0),
                ..default()
            },
            coord,
            terrain_brush.terrain_type.clone(),
        ))
        .id();

    // Add the new cell to the hashmap for easy grid lookup
    grid.insert(coord, entity);

    Ok(())
}

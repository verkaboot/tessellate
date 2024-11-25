use bevy::prelude::*;
use canvas::{tool::ToolData, SIZE};
use leafwing_input_manager::prelude::*;
use ui::interaction::OnDrag;

use crate::{
    grid::{Grid, GridCoord, GridSettings},
    terrain::{TerrainBrush, TerrainType},
};
use error::Result;

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Action {
    EraseModifier,
}

impl Action {
    pub fn input_map() -> InputMap<Self> {
        use Action::*;
        InputMap::default().with(EraseModifier, ModifierKey::Alt)
    }
}

pub fn draw_terrain(
    _trigger: Trigger<OnDrag>,
    action_state: Query<&ActionState<Action>>,
    tool_data: Res<ToolData>,
    grid_settings: Res<GridSettings>,
    mut grid: ResMut<Grid>,
    terrain_brush: Res<TerrainBrush>,
    cells: Query<&TerrainType, With<GridCoord>>,
    mut commands: Commands,
) -> Result<()> {
    let action_state = action_state.single();
    if action_state.pressed(&Action::EraseModifier) {
        return Ok(());
    }

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
    action_state: Query<&ActionState<Action>>,
    tool_data: Res<ToolData>,
    grid_settings: Res<GridSettings>,
    mut grid: ResMut<Grid>,
    mut commands: Commands,
) -> Result<()> {
    let action_state = action_state.single();
    if action_state.pressed(&Action::EraseModifier) {
        let coord = GridCoord::from_world_pos(tool_data.world_pos[0], *grid_settings);

        if let Some(cell_entity) = grid.remove(&coord) {
            commands.entity(cell_entity).despawn_recursive();
        }
    }

    Ok(())
}

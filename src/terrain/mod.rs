use bevy::{prelude::*, utils};

use canvas::{tool::ToolData, SIZE};
use error::Result;
use grid::{Grid, GridCoord, GridSettings};
use ui::widget::prelude::SelectList;
use ui_macros::SelectList;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(TerrainList::new(TerrainType::default()));
    app.insert_resource(Grid::<TerrainType>::new(GridSettings {
        cell_size: SIZE,
        offset: Vec2::ZERO,
    }));

    app.add_systems(
        Update,
        draw.map(utils::warn)
            .run_if(on_event::<event::terrain::Draw>),
    );

    app.add_systems(
        Update,
        erase
            .map(utils::warn)
            .run_if(on_event::<event::terrain::Erase>),
    );
}

#[derive(Debug, Reflect, Component, Clone)]
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
            color: Color::srgba(0.0, 0.0, 0.0, 1.0),
        }
    }
}

pub fn draw(
    tool_data: Res<ToolData>,
    mut grid: ResMut<Grid<TerrainType>>,
    terrain_list: Res<TerrainList>,
    cells: Query<&TerrainType, With<GridCoord>>,
    mut commands: Commands,
    mut event: EventWriter<event::terrain::Updated>,
) -> Result<()> {
    let terrain_coord = GridCoord::from_world_pos(tool_data.world_pos[0], grid.settings);
    let cell_pos = terrain_coord.to_world_pos(grid.settings);
    let terrain_type = terrain_list.get_selected();

    if let Some(old_cell) = (*grid).get(&terrain_coord) {
        let old_terrain_type = cells.get(old_cell.entity)?;
        if old_terrain_type == terrain_type {
            // Return early if the existing terrain type is the same
            return Ok(());
        } else {
            // Despawn the old cell
            commands.entity(old_cell.entity).despawn_recursive();
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
            Transform::from_xyz(cell_pos.x, cell_pos.y, -1.0),
            terrain_coord,
            terrain_type.clone(),
        ))
        // Debug coord text
        .with_child((
            Text2d::new(terrain_coord.to_string()),
            Transform::from_xyz(
                grid.settings.cell_size.x as f32 / 2.0,
                grid.settings.cell_size.y as f32 / 2.0,
                2.0,
            ),
        ))
        .id();

    // Add the new cell to the hashmap for easy grid lookup
    grid.insert(terrain_coord, entity, terrain_type.clone());

    event.send(event::terrain::Updated { terrain_coord });

    Ok(())
}

pub fn erase(
    tool_data: Res<ToolData>,
    mut grid: ResMut<Grid<TerrainType>>,
    mut commands: Commands,
    mut event: EventWriter<event::terrain::Updated>,
) -> Result<()> {
    let terrain_coord = GridCoord::from_world_pos(tool_data.world_pos[0], grid.settings);

    if let Some(cell) = grid.remove(&terrain_coord) {
        commands.entity(cell.entity).despawn_recursive();
        event.send(event::terrain::Updated { terrain_coord });
    }

    Ok(())
}

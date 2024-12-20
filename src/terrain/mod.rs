use bevy::{prelude::*, utils};

use canvas::{tool::ToolData, SIZE};
use error::Result;
use grid::{Grid, GridCoord, GridSettings};
use ui::widget::prelude::SelectList;
use ui_macros::SelectList;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(TerrainList::new(TerrainType::default()));
    app.insert_resource(Grid::<TerrainTile>::new(GridSettings {
        cell_size: SIZE,
        offset: SIZE.as_vec2() / 2.0,
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

pub struct TerrainTile;

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
            color: Color::srgba(0.0, 0.0, 0.0, 1.0),
        }
    }
}

pub fn draw(
    tool_data: Res<ToolData>,
    mut grid: ResMut<Grid<TerrainTile>>,
    terrain_list: Res<TerrainList>,
    cells: Query<&TerrainType, With<GridCoord>>,
    mut commands: Commands,
    mut event: EventWriter<event::terrain::Updated>,
) -> Result<()> {
    let coord = GridCoord::from_world_pos(tool_data.world_pos[0], grid.settings);
    let cell_pos = coord.to_world_pos(grid.settings);
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
            Transform::from_xyz(cell_pos.x, cell_pos.y, -1.0),
            coord,
            terrain_type.clone(),
        ))
        .id();

    // Add the new cell to the hashmap for easy grid lookup
    grid.insert(coord, entity);

    event.send(event::terrain::Updated { coord });

    Ok(())
}

pub fn erase(
    tool_data: Res<ToolData>,
    mut grid: ResMut<Grid<TerrainTile>>,
    mut commands: Commands,
    mut event: EventWriter<event::terrain::Updated>,
) -> Result<()> {
    let coord = GridCoord::from_world_pos(tool_data.world_pos[0], grid.settings);

    if let Some(cell_entity) = grid.remove(&coord) {
        commands.entity(cell_entity).despawn_recursive();
    }

    event.send(event::terrain::Updated { coord });

    Ok(())
}

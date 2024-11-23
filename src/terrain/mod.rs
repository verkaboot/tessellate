use bevy::prelude::*;
use canvas::{tool::ToolData, SIZE};
use ui::interaction::OnPress;

use crate::grid::{Grid, GridCoord, GridSettings};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GridSettings {
        cell_size: UVec2::new(SIZE.x, SIZE.y),
    })
    .init_resource::<Grid>();
}

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct TerrainType {
    label: String,
    color: Color,
}

impl Default for TerrainType {
    fn default() -> Self {
        TerrainType {
            label: "Default".to_owned(),
            color: Color::srgba(0.5, 0.5, 0.5, 1.0),
        }
    }
}

pub fn draw_terrain(
    _trigger: Trigger<OnPress>,
    tool_data: Res<ToolData>,
    grid_settings: Res<GridSettings>,
    mut grid: ResMut<Grid>,
    mut commands: Commands,
) {
    let coord = GridCoord::from_world_pos(tool_data.world_pos[0], *grid_settings);
    let cell_pos = coord.to_world_pos(*grid_settings);
    let entity = commands
        .spawn((
            Name::new("TerrainSprite"),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.2, 0.5, 0.9, 0.7),
                    custom_size: Some(SIZE.as_vec2()),
                    anchor: bevy::sprite::Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform::from_xyz(cell_pos.x, cell_pos.y, 0.0),
                ..default()
            },
        ))
        .id();
    if let Some(old_cell) = (*grid).insert(coord, entity) {
        commands.entity(old_cell).despawn_recursive();
    }
}

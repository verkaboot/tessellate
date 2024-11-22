use bevy::prelude::*;
use canvas::{tool::ToolData, SIZE};
use ui::interaction::OnPress;

use crate::grid::{GridCoord, GridSettings};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GridSettings {
        cell_size: UVec2::new(SIZE.0, SIZE.1),
    })
    .add_systems(Startup, (setup,));
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

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Cell"),
        GridCoord::new(0, 0),
        TerrainType::default(),
    ));
}

pub fn draw_terrain(
    _trigger: Trigger<OnPress>,
    tool_data: Res<ToolData>,
    grid_settings: Res<GridSettings>,
) {
    let coord = GridCoord::from_world_pos(tool_data.world_pos[0], *grid_settings);
    println!("{:?}", coord);
}

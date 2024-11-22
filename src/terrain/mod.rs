use bevy::prelude::*;

use crate::grid::GridCoord;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (setup,));
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

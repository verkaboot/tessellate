use bevy::prelude::*;

#[derive(Reflect, Resource, Debug, Copy, Clone)]
#[reflect(Resource)]
pub struct GridSettings {
    pub cell_size: UVec2,
}

#[derive(Reflect, Component, Debug, PartialEq, Eq, Clone, Copy, DerefMut, Deref)]
#[reflect(Component)]
pub struct GridCoord(IVec2);

impl GridCoord {
    pub fn new(x: i32, y: i32) -> Self {
        GridCoord(IVec2 { x, y })
    }

    pub fn from_world_pos(world_pos: Vec2, grid_settings: GridSettings) -> Self {
        (world_pos.as_ivec2() / grid_settings.cell_size.as_ivec2()).into()
    }
}

impl From<IVec2> for GridCoord {
    fn from(value: IVec2) -> Self {
        Self(value)
    }
}

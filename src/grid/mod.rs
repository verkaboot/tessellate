use bevy::{prelude::*, utils::HashMap};

#[derive(Reflect, Resource, Debug, Copy, Clone)]
#[reflect(Resource)]
pub struct GridSettings {
    pub cell_size: UVec2,
}

#[derive(Reflect, Default, Resource, Debug, Clone, Deref, DerefMut)]
#[reflect(Resource)]
pub struct Grid(HashMap<GridCoord, Entity>);

#[derive(Reflect, Component, Hash, Debug, PartialEq, Eq, Clone, Copy, DerefMut, Deref)]
#[reflect(Component)]
pub struct GridCoord(IVec2);

impl GridCoord {
    pub fn new(x: i32, y: i32) -> Self {
        GridCoord(IVec2 { x, y })
    }

    pub fn from_world_pos(world_pos: Vec2, grid_settings: GridSettings) -> Self {
        // Adjust for off-by-one when dividing negative numbers.
        let x = if world_pos.x >= 0.0 {
            world_pos.x as i32
        } else {
            (world_pos.x - grid_settings.cell_size.x as f32) as i32
        };

        let y = if world_pos.y >= 0.0 {
            world_pos.y as i32
        } else {
            (world_pos.y - grid_settings.cell_size.y as f32) as i32
        };

        (IVec2 { x, y } / grid_settings.cell_size.as_ivec2()).into()
    }

    pub fn to_world_pos(&self, grid_settings: GridSettings) -> Vec2 {
        self.0.as_vec2() * grid_settings.cell_size.as_vec2()
    }
}

impl From<IVec2> for GridCoord {
    fn from(value: IVec2) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod coord {
    use super::*;

    fn grid_settings() -> GridSettings {
        GridSettings {
            cell_size: UVec2 { x: 8, y: 8 },
        }
    }

    #[test]
    fn from_world_at_0() {
        assert_eq!(
            GridCoord::from_world_pos(Vec2::new(0.0, 0.0), grid_settings()),
            GridCoord::new(0, 0)
        );
    }

    #[test]
    fn from_world_positive() {
        assert_eq!(
            GridCoord::from_world_pos(Vec2::new(38.2, 27.9), grid_settings()),
            GridCoord::new(4, 3)
        );
    }

    #[test]
    fn from_world_negative() {
        assert_eq!(
            GridCoord::from_world_pos(Vec2::new(-7.0, -1.0), grid_settings()),
            GridCoord::new(-1, -1)
        );
    }

    #[test]
    fn to_world_0() {
        assert_eq!(
            GridCoord::new(0, 0).to_world_pos(grid_settings()),
            Vec2::new(0.0, 0.0)
        );
    }

    #[test]
    fn to_world_positive() {
        assert_eq!(
            GridCoord::new(3, 7).to_world_pos(grid_settings()),
            Vec2::new(24.0, 56.0)
        );
    }

    #[test]
    fn to_world_negative() {
        assert_eq!(
            GridCoord::new(-1, -1).to_world_pos(grid_settings()),
            Vec2::new(-8.0, -8.0)
        );
    }
}

use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Clone, Copy, DerefMut, Deref, Reflect)]
#[reflect(Component)]
pub struct GridCoord(IVec2);

impl GridCoord {
    pub fn new(x: i32, y: i32) -> Self {
        GridCoord(IVec2 { x, y })
    }
}

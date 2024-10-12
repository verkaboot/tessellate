use bevy::{prelude::*, render::extract_resource::ExtractResource};

#[derive(Debug, Resource, Clone, Copy, ExtractResource, Deref, DerefMut)]
pub struct BrushSize(pub f32);

impl Into<f32> for BrushSize {
    fn into(self) -> f32 {
        self.0
    }
}

impl From<f32> for BrushSize {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Debug, Resource, Clone, Copy, ExtractResource, Deref, DerefMut)]
pub struct BrushColor(pub Color);

impl BrushColor {
    pub fn new(color: LinearRgba) -> Self {
        BrushColor(color.into())
    }
}

#[derive(Default, Debug, Resource, Clone, Copy, ExtractResource)]
pub enum BrushType {
    #[default]
    Normal,
    Erase,
}

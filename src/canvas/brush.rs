use bevy::{prelude::*, render::extract_resource::ExtractResource};

use crate::ui::interaction::AsVal;

#[derive(Debug, Resource, Clone, Copy, ExtractResource, Deref, DerefMut)]
pub struct BrushSize(pub f32);

impl AsVal for BrushSize {
    fn as_val(&self) -> Val {
        Val::Px(self.0)
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

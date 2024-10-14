use bevy::{prelude::*, render::extract_resource::ExtractResource};
use ui::macros::SliderValue;
use ui::widget::prelude::SliderValue;

#[derive(Debug, Resource, Clone, Copy, ExtractResource, Deref, DerefMut, SliderValue)]
pub struct BrushSize(pub f32);

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

use bevy::{prelude::*, render::extract_resource::ExtractResource};

#[derive(Debug, Resource, Clone, Copy, ExtractResource, Deref, DerefMut)]
pub struct BrushSize(pub f32);

#[derive(Debug, Resource, Clone, Copy, ExtractResource, Deref, DerefMut)]
pub struct BrushColor(Color);

impl BrushColor {
    pub fn new(color: LinearRgba) -> Self {
        let premultiplied_color = Color::srgba(
            color.red * color.alpha,
            color.green * color.alpha,
            color.blue * color.alpha,
            color.alpha,
        );
        BrushColor(premultiplied_color)
    }
}

#[derive(Default, Debug, Resource, Clone, Copy, ExtractResource)]
pub enum BrushType {
    #[default]
    Normal,
    Erase,
}

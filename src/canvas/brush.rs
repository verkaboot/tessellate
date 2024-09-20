use bevy::{prelude::*, render::extract_resource::ExtractResource};

#[derive(Resource, Clone, Copy, ExtractResource)]
pub struct BrushData {
    pub size: f32,
    pub color: Color,
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(BrushData {
        size: 1.0,
        color: Color::srgb(1.0, 0.3, 0.7),
    })
}

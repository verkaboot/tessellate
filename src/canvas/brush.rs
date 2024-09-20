use bevy::{prelude::*, render::extract_resource::ExtractResource};

#[derive(Debug, Resource, Clone, Copy, ExtractResource, Deref, DerefMut)]
pub struct BrushSize(pub f32);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(BrushSize(1.0))
}

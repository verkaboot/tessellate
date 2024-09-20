use bevy::{prelude::*, render::extract_resource::ExtractResource};

#[derive(Debug, Resource, Clone, Copy, ExtractResource, Deref, DerefMut)]
pub struct BrushSize(pub f32);

#[derive(Debug, Resource, Clone, Copy, ExtractResource, Deref, DerefMut)]
pub struct BrushColor(pub Color);

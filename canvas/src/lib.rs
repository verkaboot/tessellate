mod bind_groups;
pub mod brush;
mod compute;
pub mod mouse;
mod node;
mod pipeline;
pub mod sprite;

use bevy::{prelude::*, utils};
use brush::{BrushColor, BrushSize};
use compute::CanvasComputePlugin;

pub const SIZE: (u32, u32) = (256, 256);
const SHADER_ASSET_PATH: &str = "shaders/canvas.wgsl";
const WORKGROUP_SIZE: u32 = 8;

pub fn plugin(app: &mut App) {
    app.add_plugins((CanvasComputePlugin, sprite::plugin))
        .insert_resource(BrushSize(8.0))
        .insert_resource(BrushColor::new(
            Color::linear_rgba(1.0, 0.0, 0.0, 1.0).to_linear(),
        ))
        .add_systems(PreStartup, (sprite::setup, mouse::setup))
        .add_systems(Update, mouse::update_position.map(utils::dbg));
}

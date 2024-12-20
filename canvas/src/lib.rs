pub mod bind_groups;
pub mod brush;
mod compute;
mod pipeline;
mod render_node;
pub mod tool;

use bevy::{prelude::*, utils};
use brush::{BrushColor, BrushHardness, BrushSize};
use compute::CanvasComputePlugin;

pub const SIZE: UVec2 = UVec2::new(256, 256);
const SHADER_ASSET_PATH: &str = "shaders/canvas.wgsl";
const WORKGROUP_SIZE: u32 = 8;

pub fn plugin(app: &mut App) {
    app.register_type::<bind_groups::CanvasSprite>()
        .add_plugins(CanvasComputePlugin)
        .insert_resource(BrushSize(8.0))
        .insert_resource(BrushHardness(0.5))
        .insert_resource(BrushColor::new(
            Color::linear_rgba(1.0, 0.0, 0.0, 1.0).to_linear(),
        ))
        .add_systems(PreStartup, tool::setup)
        .add_systems(Update, tool::update_position.map(utils::dbg));
}

mod bind_groups;
mod brush;
mod compute;
pub mod mouse;
mod node;
mod pipeline;
pub mod sprite;

use bevy::{prelude::*, utils::warn};
use compute::CanvasComputePlugin;

pub const SIZE: (u32, u32) = (1920 * 3, 1920 * 3);
const SHADER_ASSET_PATH: &str = "shaders/canvas.wgsl";
const WORKGROUP_SIZE: u32 = 8;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(CanvasComputePlugin)
        .add_systems(Startup, (sprite::setup, mouse::setup, brush::setup))
        .add_systems(
            Update,
            (mouse::update_position.map(warn), mouse::update_button_state),
        );
}

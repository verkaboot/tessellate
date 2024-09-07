mod bind_groups;
mod compute;
mod node;
mod pipeline;
mod sprite;

use bevy::prelude::*;
use compute::CanvasComputePlugin;

pub const SIZE: (u32, u32) = (1920 * 3, 1920 * 3);
const SHADER_ASSET_PATH: &str = "shaders/canvas.wgsl";
const WORKGROUP_SIZE: u32 = 8;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(CanvasComputePlugin)
        .add_systems(Startup, sprite::setup)
        .add_systems(
            Update,
            (
                sprite::update_mouse_position,
                sprite::update_mouse_button_state,
            ),
        );
}

mod compute;
mod sprite;

use bevy::prelude::*;
use compute::GameOfLifeComputePlugin;

pub const SIZE: (u32, u32) = (1280, 720);
const SHADER_ASSET_PATH: &str = "shaders/game_of_life.wgsl";
const WORKGROUP_SIZE: u32 = 8;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(GameOfLifeComputePlugin)
        .add_systems(Startup, sprite::setup)
        .add_systems(Update, sprite::switch_textures);
}

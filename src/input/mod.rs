pub mod camera;
pub mod paint;
pub mod terrain;

use bevy::prelude::*;
use leafwing_input_manager::{plugin::InputManagerPlugin, prelude::ClashStrategy};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        InputManagerPlugin::<camera::CameraMovement>::default(),
        InputManagerPlugin::<terrain::Action>::default(),
    ))
    .insert_resource(ClashStrategy::PrioritizeLongest);
}

pub mod camera;
pub mod terrain;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<terrain::Draw>();
    app.add_event::<terrain::Erase>();
    app.add_event::<camera::Pan>();
    app.add_event::<camera::Zoom>();
}

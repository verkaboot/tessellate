pub mod button;
pub mod camera;
pub mod paint;
pub mod terrain;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<camera::Pan>();
    app.add_event::<camera::Zoom>();
    app.add_event::<button::SetBrush>();
    app.add_event::<button::SelectLayer>();
    app.add_event::<button::ChangeColor>();
    app.add_event::<paint::ActivateTool>();
    app.add_event::<paint::StopTool>();
    app.add_event::<terrain::Draw>();
    app.add_event::<terrain::Erase>();
}

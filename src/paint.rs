use bevy::prelude::*;
use canvas::tool::ToolData;

use crate::event;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, tool);
    app.add_systems(Update, tool_stop);
}

pub fn tool(mut event: EventReader<event::paint::ActivateTool>, mut tool_data: ResMut<ToolData>) {
    for event::paint::ActivateTool in event.read() {
        tool_data.tool_active = true;
    }
}

pub fn tool_stop(mut event: EventReader<event::paint::StopTool>, mut tool_data: ResMut<ToolData>) {
    for event::paint::StopTool in event.read() {
        tool_data.tool_active = false;
    }
}

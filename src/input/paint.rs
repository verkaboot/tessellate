use bevy::prelude::*;
use canvas::{
    brush::BrushColor,
    sprite::CanvasImages,
    tool::{ToolData, ToolType},
};
use ui::interaction::{OnPress, OnRelease};

pub fn set_brush(brush: &ToolType) -> impl Fn(Trigger<OnPress>, ResMut<ToolData>) + '_ {
    |_trigger: Trigger<OnPress>, mut tool_data: ResMut<ToolData>| {
        tool_data.tool_type = *brush;
    }
}

pub fn select_layer(_trigger: Trigger<OnPress>, mut canvas: ResMut<CanvasImages>) {
    canvas.active_layer += 1;
}

pub fn activate_tool(_trigger: Trigger<OnPress>, mut tool_data: ResMut<ToolData>) {
    tool_data.tool_active = true;
}

pub fn stop_tool(_trigger: Trigger<OnRelease>, mut tool_data: ResMut<ToolData>) {
    tool_data.tool_active = false;
}

pub fn change_color(_trigger: Trigger<OnRelease>, mut brush_color: ResMut<BrushColor>) {
    brush_color.0 = Color::rotate_hue(&brush_color.0, 10.0);
}

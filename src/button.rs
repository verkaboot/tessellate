use bevy::prelude::*;
use canvas::{brush::BrushColor, sprite::CanvasImages, tool::ToolData};

use crate::event;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, set_brush);
    app.add_systems(Update, select_layer);
    app.add_systems(Update, change_color);
}

pub fn set_brush(mut event: EventReader<event::button::SetBrush>, mut tool_data: ResMut<ToolData>) {
    for event::button::SetBrush { brush } in event.read() {
        tool_data.tool_type = *brush;
    }
}

pub fn select_layer(
    mut event: EventReader<event::button::SelectLayer>,
    mut canvas: ResMut<CanvasImages>,
) {
    for event::button::SelectLayer in event.read() {
        canvas.active_layer += 1;
    }
}

pub fn change_color(
    mut event: EventReader<event::button::ChangeColor>,
    mut brush_color: ResMut<BrushColor>,
) {
    for event::button::ChangeColor in event.read() {
        brush_color.0 = Color::rotate_hue(&brush_color.0, 10.0);
    }
}

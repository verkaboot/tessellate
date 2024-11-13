use bevy::prelude::*;
use canvas::mouse::ToolType;
use ui::icon::Icon;
use ui::interaction::{
    trigger_on_resource_updated, trigger_watch_resource_init, OnPress, OnRelease,
};
use ui::widget::color_picker::{ColorPickerWidget, HsvBoxMaterial, HueWheelMaterial};
use ui::widget::prelude::*;

use canvas::{
    brush::{BrushColor, BrushSize, BrushHardness},
    mouse::ToolData,
    sprite::CanvasImages,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            trigger_on_resource_updated::<BrushSize>,
            trigger_watch_resource_init::<BrushSize>,
            trigger_on_resource_updated::<BrushHardness>,
            trigger_watch_resource_init::<BrushHardness>,
        ),
    );
}

pub fn setup(
    mut commands: Commands,
    hue_wheel_material: ResMut<Assets<HueWheelMaterial>>,
    hsv_box_material: ResMut<Assets<HsvBoxMaterial>>,
) {
    commands.ui_root().with_children(|ui_root| {
        ui_root.panel(PanelDirection::Wide);
        ui_root.flex().with_children(|flex| {
            flex.panel(PanelDirection::Tall).with_children(|side_bar| {
                side_bar
                    .button()
                    .add(Icon::Brush)
                    .observe(set_brush(&ToolType::Paint));
                side_bar
                    .button()
                    .add(Icon::Eraser)
                    .observe(set_brush(&ToolType::Erase));
                side_bar.button().add(Icon::Layer).observe(select_layer);
            });
            flex.canvas().observe(activate_tool).observe(stop_tool);
            flex.panel(PanelDirection::Tall)
                .with_children(|side_bar_right| {
                    side_bar_right
                        .button()
                        .add(Icon::ColorPicker)
                        .observe(change_color);
                    side_bar_right.color_picker(hue_wheel_material, hsv_box_material);
                    side_bar_right.slider::<BrushSize>("Brush Size", 1.0, 200.0);
                    side_bar_right.slider::<BrushHardness>("Brush Hardness", 0.1, 1.0);
                });
        });
        ui_root.panel(PanelDirection::Wide);
    });
}

fn set_brush(brush: &ToolType) -> impl Fn(Trigger<OnPress>, ResMut<ToolData>) + '_ {
    |_trigger: Trigger<OnPress>, mut tool_data: ResMut<ToolData>| {
        tool_data.tool_type = *brush;
    }
}
fn select_layer(_trigger: Trigger<OnPress>, mut canvas: ResMut<CanvasImages>) {
    canvas.active_layer += 1;
}

fn activate_tool(_trigger: Trigger<OnPress>, mut tool_data: ResMut<ToolData>) {
    tool_data.tool_active = true;
}

fn stop_tool(_trigger: Trigger<OnRelease>, mut tool_data: ResMut<ToolData>) {
    tool_data.tool_active = false;
}

fn change_color(_trigger: Trigger<OnRelease>, mut brush_color: ResMut<BrushColor>) {
    brush_color.0 = Color::rotate_hue(&brush_color.0, 10.0);
}

use bevy::prelude::*;
use canvas::tool::ToolType;
use ui::icon::Icon;
use ui::interaction::{
    trigger_on_resource_updated, trigger_watch_resource_init, OnPress, OnRelease,
};
use ui::widget::color_picker::{ColorPickerWidget, HsvBoxMaterial, HueWheelMaterial};
use ui::widget::prelude::*;

use canvas::{
    brush::{BrushColor, BrushHardness, BrushSize},
    sprite::CanvasImages,
    tool::ToolData,
};

use crate::terrain::draw_terrain;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            trigger_on_resource_updated::<BrushSize>,
            trigger_watch_resource_init::<BrushSize>,
            trigger_on_resource_updated::<BrushHardness>,
            trigger_watch_resource_init::<BrushHardness>,
            trigger_on_resource_updated::<CurrentState<UiMode>>,
            trigger_watch_resource_init::<CurrentState<UiMode>>,
            watch_state::<UiMode>,
        ),
    );
}

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
enum UiMode {
    Paint,
    Terrain,
}

impl std::fmt::Display for UiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UiMode::Paint => "Paint",
                UiMode::Terrain => "Terrain",
            }
        )
    }
}

impl RootState for UiMode {}

fn top_bar(root: &mut ChildBuilder) {
    root.panel(PanelDirection::Wide).with_children(|top_panel| {
        top_panel
            .button()
            .add(Icon::PaintView)
            .observe(set_root::<UiMode>(UiMode::Paint));
        top_panel
            .button()
            .add(Icon::TerrainView)
            .observe(set_root::<UiMode>(UiMode::Terrain));
        top_panel.text::<CurrentState<UiMode>>("Mode\n");
    });
}

pub fn setup(
    mut commands: Commands,
    hue_wheel_material: ResMut<Assets<HueWheelMaterial>>,
    hsv_box_material: ResMut<Assets<HsvBoxMaterial>>,
) {
    commands.insert_resource(CurrentState(UiMode::Paint));
    commands.ui_root(UiMode::Paint).with_children(|root| {
        top_bar(root);
        root.flex().with_children(|flex| {
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
                    // TODO: Make a way to not need to pass in materials as arguments
                    side_bar_right.color_picker(hue_wheel_material, hsv_box_material);
                    side_bar_right.slider::<BrushSize>("Brush Size", 1.0, 200.0);
                    side_bar_right.slider::<BrushHardness>("Brush Hardness", 0.1, 1.0);
                });
        });
        root.panel(PanelDirection::Wide);
    });

    commands.ui_root(UiMode::Terrain).with_children(|root| {
        top_bar(root);
        root.flex().with_children(|flex| {
            flex.panel(PanelDirection::Tall);
            flex.canvas().observe(draw_terrain);
            flex.panel(PanelDirection::Tall);
        });
        root.panel(PanelDirection::Wide);
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

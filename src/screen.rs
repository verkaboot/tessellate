use bevy::prelude::*;
use ui::icon::Icon;
use ui::interaction::{
    trigger_on_resource_updated, trigger_watch_resource_init, OnPress, OnRelease,
};
use ui::widget::prelude::*;

use canvas::{
    brush::{BrushColor, BrushSize, BrushType},
    mouse::MouseData,
    sprite::CanvasImages,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            trigger_on_resource_updated::<BrushSize>,
            trigger_watch_resource_init::<BrushSize>,
        ),
    );
}

pub fn setup(mut commands: Commands) {
    commands.ui_root().with_children(|ui_root| {
        ui_root.panel(PanelDirection::Wide);
        ui_root.flex().with_children(|flex| {
            flex.panel(PanelDirection::Tall).with_children(|side_bar| {
                side_bar
                    .button()
                    .add(Icon::Brush)
                    .observe(set_brush(&BrushType::Normal));
                side_bar
                    .button()
                    .add(Icon::Eraser)
                    .observe(set_brush(&BrushType::Erase));
                side_bar.button().add(Icon::Layer).observe(select_layer);
            });
            flex.canvas().observe(start_painting).observe(stop_painting);
            flex.panel(PanelDirection::Tall)
                .with_children(|side_bar_right| {
                    side_bar_right
                        .button()
                        .add(Icon::ColorPicker)
                        .observe(change_color);
                    side_bar_right.slider::<BrushSize>("Brush Size");
                });
        });
        ui_root.panel(PanelDirection::Wide);
    });
}

fn set_brush(brush: &BrushType) -> impl Fn(Trigger<OnPress>, ResMut<BrushType>) + '_ {
    |_trigger: Trigger<OnPress>, mut brush_type: ResMut<BrushType>| {
        *brush_type = *brush;
    }
}
fn select_layer(_trigger: Trigger<OnPress>, mut canvas: ResMut<CanvasImages>) {
    canvas.active_layer += 1;
}

fn start_painting(_trigger: Trigger<OnPress>, mut mouse_data: ResMut<MouseData>) {
    mouse_data.left_button_pressed = true;
}

fn stop_painting(_trigger: Trigger<OnRelease>, mut mouse_data: ResMut<MouseData>) {
    mouse_data.left_button_pressed = false;
}

fn change_color(_trigger: Trigger<OnRelease>, mut brush_color: ResMut<BrushColor>) {
    brush_color.0 = Color::rotate_hue(&brush_color.0, 10.0);
}

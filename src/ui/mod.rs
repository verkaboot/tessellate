mod icon;
mod interaction;
mod theme;
mod widget;

use bevy::prelude::*;
use icon::Icon;
use interaction::{OnPress, OnRelease};
use widget::{Containers, PanelDirection, Widget};

use crate::canvas::{brush::BrushType, mouse::MouseData, sprite::CanvasImages};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin)
        .add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
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
            flex.panel(PanelDirection::Tall);
        });
        ui_root.panel(PanelDirection::Wide);
    });
}

fn set_brush(brush: &BrushType) -> impl Fn(Trigger<OnPress>, ResMut<BrushType>) + '_ {
    dbg!(brush);
    |_trigger: Trigger<OnPress>, mut brush_type: ResMut<BrushType>| {
        *brush_type = *brush;
        dbg!(brush_type);
    }
}
fn select_layer(_trigger: Trigger<OnPress>, mut canvas: ResMut<CanvasImages>) {
    canvas.active_layer += 1;
    println!("{}", canvas.active_layer);
}

fn start_painting(_trigger: Trigger<OnPress>, mut mouse_data: ResMut<MouseData>) {
    mouse_data.left_button_pressed = true;
}

fn stop_painting(_trigger: Trigger<OnRelease>, mut mouse_data: ResMut<MouseData>) {
    mouse_data.left_button_pressed = false;
}

mod icon;
mod interaction;
mod theme;
mod widget;

use bevy::prelude::*;
use icon::Icon;
use interaction::{OnPress, OnRelease};
use widget::{Containers, PanelDirection, Widget};

use crate::canvas::{brush::BrushType, mouse::MouseData};

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
                    .observe(set_brush(&BrushType::Normal))
                    .observe(test_observer);
                side_bar.button().observe(set_brush(&BrushType::Erase));
            });
            flex.canvas().observe(start_painting).observe(stop_painting);
            flex.panel(PanelDirection::Tall);
        });
        ui_root.panel(PanelDirection::Wide);
    });
}

fn test_observer(_trigger: Trigger<OnPress>) {
    println!("Helloi");
}

fn set_brush(brush: &BrushType) -> impl Fn(Trigger<OnPress>, ResMut<BrushType>) + '_ {
    dbg!(brush);
    |_trigger: Trigger<OnPress>, mut brush_type: ResMut<BrushType>| {
        *brush_type = *brush;
        dbg!(brush_type);
    }
}

fn start_painting(_trigger: Trigger<OnPress>, mut mouse_data: ResMut<MouseData>) {
    mouse_data.left_button_pressed = true;
}

fn stop_painting(_trigger: Trigger<OnRelease>, mut mouse_data: ResMut<MouseData>) {
    mouse_data.left_button_pressed = false;
}

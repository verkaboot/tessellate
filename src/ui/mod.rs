mod icon;
mod interaction;
mod theme;
mod widget;

use bevy::prelude::*;
use icon::Icon;
use interaction::{OnPress, OnRelease};
use widget::{Containers, PanelDirection, Widget};

use crate::canvas::mouse::MouseData;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin)
        .add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    commands.ui_root().with_children(|ui_root| {
        ui_root
            .panel(PanelDirection::Wide)
            .with_children(|top_bar| {
                top_bar.button().add(Icon::Brush);
            });
        ui_root.flex().with_children(|flex| {
            flex.panel(PanelDirection::Tall);
            flex.canvas().observe(start_painting).observe(stop_painting);
            flex.panel(PanelDirection::Tall);
        });
        ui_root.panel(PanelDirection::Wide);
    });
}

fn start_painting(_trigger: Trigger<OnPress>, mut mouse_data: ResMut<MouseData>) {
    mouse_data.left_button_pressed = true;
}

fn stop_painting(_trigger: Trigger<OnRelease>, mut mouse_data: ResMut<MouseData>) {
    mouse_data.left_button_pressed = false;
}

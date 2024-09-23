mod icon;
mod theme;
mod widget;

use bevy::prelude::*;
use icon::Icon;
use widget::{Containers, Widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    commands.ui_root().with_children(|ui_root| {
        ui_root.top_bar().with_children(|top_bar| {
            top_bar.button(Icon::Brush);
        });
        ui_root.canvas();
        ui_root.bottom_bar();
    });
}

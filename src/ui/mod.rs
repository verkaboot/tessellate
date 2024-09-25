mod interaction;
mod theme;
mod widget;

use bevy::prelude::*;
use widget::{Containers, PanelDirection, Widget};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin)
        .add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    commands.ui_root().with_children(|ui_root| {
        ui_root
            .panel(PanelDirection::Wide)
            .with_children(|top_bar| {
                top_bar.button();
            });
        ui_root.flex().with_children(|flex| {
            flex.panel(PanelDirection::Tall);
            flex.canvas();
            flex.panel(PanelDirection::Tall);
        });
        ui_root.panel(PanelDirection::Wide);
    });
}

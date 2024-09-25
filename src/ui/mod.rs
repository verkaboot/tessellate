mod interaction;
mod theme;
mod widget;

use bevy::prelude::*;
use widget::{Containers, Icon, IconType, PanelDirection, Widget};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin)
        .add_systems(Startup, setup);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.ui_root().with_children(|ui_root| {
        ui_root
            .panel(PanelDirection::Wide)
            .with_children(|top_bar| {
                top_bar.button().with_children(|brush_button| {
                    brush_button.icon(asset_server, IconType::Brush);
                });
            });
        ui_root.flex().with_children(|flex| {
            flex.panel(PanelDirection::Tall);
            flex.canvas();
            flex.panel(PanelDirection::Tall);
        });
        ui_root.panel(PanelDirection::Wide);
    });
}

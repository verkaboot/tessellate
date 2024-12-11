pub mod icon;
pub mod interaction;
pub mod theme;
pub mod widget;

use bevy::prelude::*;

pub mod macros {
    pub use ui_macros::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((widget::plugin, interaction::plugin, event::plugin));
}

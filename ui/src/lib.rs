pub mod icon;
pub mod theme;
pub mod widget;

use bevy::prelude::*;

pub mod macros {
    pub use ui_macros::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(widget::plugin);
}

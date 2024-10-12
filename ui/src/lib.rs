pub mod icon;
pub mod interaction;
pub mod theme;
pub mod widget;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}

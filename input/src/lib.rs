pub mod interaction;
pub mod trigger;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((trigger::plugin, interaction::plugin));
}

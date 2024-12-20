pub mod interaction;
pub mod trigger;

use bevy::prelude::*;
use KeyCode::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((trigger::plugin, interaction::plugin));
}

pub mod camera {
    use super::*;
    pub const POINTER_BUTTON: PointerButton = PointerButton::Middle;
    pub const ZOOM: KeyCode = ControlLeft;
}

pub mod terrain {
    use super::*;
    pub const DRAW_BUTTON: PointerButton = PointerButton::Primary;
    pub const ERASE_BUTTON: PointerButton = PointerButton::Secondary;
}

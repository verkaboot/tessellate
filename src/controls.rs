use bevy::prelude::{KeyCode::*, *};

pub mod camera {
    use super::*;
    pub const PAN: KeyCode = AltLeft;
    pub const POINTER_BUTTON: PointerButton = PointerButton::Middle;
    pub const ZOOM: KeyCode = ControlLeft;
}

pub mod terrain {
    use super::*;
    pub const DRAW_BUTTON: PointerButton = PointerButton::Primary;
    pub const ERASE_BUTTON: PointerButton = PointerButton::Secondary;
}

use bevy::prelude::{KeyCode::*, *};

pub mod camera {
    use super::*;
    pub const PAN: KeyCode = AltLeft;
    pub const POINTER_BUTTON: PointerButton = PointerButton::Secondary;
    pub const ZOOM: KeyCode = ControlLeft;
}

pub mod terrain {
    use super::*;
    pub const ERASE_MODIFIER: KeyCode = AltLeft;
}

pub struct KeyChord {}

// I need to make a trigger that checks the input and
// then dispatches an event (or changes the state?)
// and then the systems watch that input state to
// determine if they should run.

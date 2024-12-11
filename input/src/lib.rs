pub mod interaction;
pub mod trigger;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((trigger::plugin, interaction::plugin));
}

pub fn key_pressed(key: KeyCode) -> impl Fn(Res<ButtonInput<KeyCode>>) -> bool {
    move |keyboard_input: Res<ButtonInput<KeyCode>>| keyboard_input.pressed(key)
}

pub fn mouse_pressed(button: MouseButton) -> impl Fn(Res<ButtonInput<MouseButton>>) -> bool {
    move |mouse_input: Res<ButtonInput<MouseButton>>| mouse_input.pressed(button)
}

use bevy::prelude::*;

use crate::controls;

#[derive(Event)]
pub struct Pan {
    pub delta: Vec2,
}

pub fn pan(
    trigger: Trigger<Pointer<Drag>>,
    mut msg: EventWriter<Pan>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if trigger.button == controls::camera::POINTER_BUTTON && !input.pressed(controls::camera::ZOOM)
    {
        msg.send(Pan {
            delta: trigger.delta,
        });
    }
}

#[derive(Event)]
pub struct Zoom {
    pub delta: Vec2,
}

pub fn zoom(
    trigger: Trigger<Pointer<Drag>>,
    mut msg: EventWriter<Zoom>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if trigger.button == controls::camera::POINTER_BUTTON && input.pressed(controls::camera::ZOOM) {
        msg.send(Zoom {
            delta: trigger.delta,
        });
    }
}

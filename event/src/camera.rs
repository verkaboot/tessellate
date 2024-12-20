use bevy::prelude::*;

#[derive(Event)]
pub struct Pan {
    pub delta: Vec2,
}

pub fn pan(
    trigger: Trigger<Pointer<Drag>>,
    mut msg: EventWriter<Pan>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if trigger.button == input::camera::POINTER_BUTTON && !input.pressed(input::camera::ZOOM) {
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
    if trigger.button == input::camera::POINTER_BUTTON && input.pressed(input::camera::ZOOM) {
        msg.send(Zoom {
            delta: trigger.delta,
        });
    }
}

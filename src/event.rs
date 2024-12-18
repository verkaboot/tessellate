use bevy::prelude::*;

use crate::controls;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<DrawTerrain>();
    app.add_event::<EraseTerrain>();
    app.add_event::<PanCamera>();
    app.add_event::<ZoomCamera>();
}

#[derive(Event)]
pub struct DrawTerrain;

pub fn draw_terrain(trigger: Trigger<Pointer<Drag>>, mut msg: EventWriter<DrawTerrain>) {
    if trigger.button == controls::terrain::DRAW_BUTTON {
        msg.send(DrawTerrain);
    }
}

#[derive(Event)]
pub struct EraseTerrain;

pub fn erase_terrain(trigger: Trigger<Pointer<Drag>>, mut msg: EventWriter<EraseTerrain>) {
    if trigger.button == controls::terrain::ERASE_BUTTON {
        msg.send(EraseTerrain);
    }
}

#[derive(Event)]
pub struct PanCamera {
    pub delta: Vec2,
}

pub fn pan_camera(
    trigger: Trigger<Pointer<Drag>>,
    mut msg: EventWriter<PanCamera>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if trigger.button == controls::camera::POINTER_BUTTON && !input.pressed(controls::camera::ZOOM)
    {
        msg.send(PanCamera {
            delta: trigger.delta,
        });
    }
}

#[derive(Event)]
pub struct ZoomCamera {
    pub delta: Vec2,
}

pub fn zoom_camera(
    trigger: Trigger<Pointer<Drag>>,
    mut msg: EventWriter<ZoomCamera>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if trigger.button == controls::camera::POINTER_BUTTON && input.pressed(controls::camera::ZOOM) {
        msg.send(ZoomCamera {
            delta: trigger.delta,
        });
    }
}

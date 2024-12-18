use bevy::prelude::*;

use crate::controls;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<DrawTerrain>();
    app.add_event::<PanCamera>();
}

#[derive(Event)]
pub struct DrawTerrain;

pub fn draw_terrain(trigger: Trigger<Pointer<Drag>>, mut msg: EventWriter<DrawTerrain>) {
    if trigger.button == controls::terrain::DRAW_BUTTON {
        msg.send(DrawTerrain);
    }
}

#[derive(Event)]
pub struct PanCamera {
    pub delta: Vec2,
}

pub fn pan_camera(trigger: Trigger<Pointer<Drag>>, mut msg: EventWriter<PanCamera>) {
    if trigger.button == controls::camera::POINTER_BUTTON {
        msg.send(PanCamera {
            delta: trigger.delta,
        });
    }
}

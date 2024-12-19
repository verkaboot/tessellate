use bevy::prelude::*;

use crate::controls;

#[derive(Event)]
pub struct Draw;

pub fn draw(trigger: Trigger<Pointer<Drag>>, mut msg: EventWriter<Draw>) {
    if trigger.button == controls::terrain::DRAW_BUTTON {
        msg.send(Draw);
    }
}

#[derive(Event)]
pub struct Erase;

pub fn erase(trigger: Trigger<Pointer<Drag>>, mut msg: EventWriter<Erase>) {
    if trigger.button == controls::terrain::ERASE_BUTTON {
        msg.send(Erase);
    }
}

use bevy::prelude::*;
use grid::GridCoord;

#[derive(Event)]
pub struct Draw;

pub fn draw(trigger: Trigger<Pointer<Drag>>, mut msg: EventWriter<Draw>) {
    if trigger.button == input::terrain::DRAW_BUTTON {
        msg.send(Draw);
    }
}

#[derive(Event)]
pub struct Erase;

pub fn erase(trigger: Trigger<Pointer<Drag>>, mut msg: EventWriter<Erase>) {
    if trigger.button == input::terrain::ERASE_BUTTON {
        msg.send(Erase);
    }
}

#[derive(Event, Debug)]
pub struct Updated {
    pub terrain_coord: GridCoord,
}

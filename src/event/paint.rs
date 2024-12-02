use bevy::prelude::*;

#[derive(Event)]
pub struct ActivateTool;

pub fn activate_tool(trigger: Trigger<Pointer<Down>>, mut msg: EventWriter<ActivateTool>) {
    if trigger.button == PointerButton::Primary {
        msg.send(ActivateTool);
    }
}

#[derive(Event)]
pub struct StopTool;

pub fn stop_tool(trigger: Trigger<Pointer<Up>>, mut msg: EventWriter<StopTool>) {
    if trigger.button == PointerButton::Primary {
        msg.send(StopTool);
    }
}

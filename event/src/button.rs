use bevy::prelude::*;

use canvas::tool::ToolType;

#[derive(Event)]
pub struct SetBrush {
    pub brush: ToolType,
}

pub fn set_brush(brush: ToolType) -> impl Fn(Trigger<Pointer<Click>>, EventWriter<SetBrush>) {
    move |trigger: Trigger<Pointer<Click>>, mut msg: EventWriter<SetBrush>| {
        if trigger.button == PointerButton::Primary {
            msg.send(SetBrush { brush });
        }
    }
}

#[derive(Event)]
pub struct SelectLayer;

pub fn select_layer(trigger: Trigger<Pointer<Click>>, mut msg: EventWriter<SelectLayer>) {
    if trigger.button == PointerButton::Primary {
        msg.send(SelectLayer);
    }
}

#[derive(Event)]
pub struct ChangeColor;

pub fn change_color(trigger: Trigger<Pointer<Click>>, mut msg: EventWriter<ChangeColor>) {
    if trigger.button == PointerButton::Primary {
        msg.send(ChangeColor);
    }
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<PanCamera>();
}

#[derive(Event)]
pub struct PanCamera {
    pub delta: Vec2,
}

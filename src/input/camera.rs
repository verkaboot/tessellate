use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum CameraMovement {
    Pan,
    Zoom,
    ZoomModifier,
    ZoomWheel,
}

impl Actionlike for CameraMovement {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            CameraMovement::Pan => InputControlKind::Button,
            CameraMovement::Zoom => InputControlKind::Button,
            CameraMovement::ZoomModifier => InputControlKind::Button,
            CameraMovement::ZoomWheel => InputControlKind::Axis,
        }
    }
}

impl CameraMovement {
    pub fn input_map() -> InputMap<Self> {
        use CameraMovement::*;
        InputMap::default()
            .with(ZoomModifier, ModifierKey::Alt)
            .with(Pan, MouseButton::Right)
            .with(
                Zoom,
                ButtonlikeChord::from_single(MouseButton::Right).with(ModifierKey::Alt),
            )
            .with_axis(ZoomWheel, MouseScrollAxis::Y)
    }
}

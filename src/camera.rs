use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::canvas::SIZE;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<CameraMovement>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (pan, zoom));
}

pub fn setup(mut commands: Commands) {
    let input_map = InputMap::default()
        .with(CameraMovement::ActivatePan, MouseButton::Right)
        .with_axis(CameraMovement::Zoom, MouseScrollAxis::Y)
        .with_dual_axis(CameraMovement::Pan, MouseMove::default());

    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(
                SIZE.0 as f32 / 2.0,
                SIZE.1 as f32 / 2.0,
                0.0,
            )),
            ..default()
        },
        InputManagerBundle::with_map(input_map),
        IsDefaultUiCamera,
    ));
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
enum CameraMovement {
    ActivatePan,
    Pan,
    Zoom,
}

impl Actionlike for CameraMovement {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            CameraMovement::ActivatePan => InputControlKind::Button,
            CameraMovement::Pan => InputControlKind::DualAxis,
            CameraMovement::Zoom => InputControlKind::Axis,
        }
    }
}

fn pan(mut query: Query<(&mut Transform, &ActionState<CameraMovement>), With<Camera2d>>) {
    const CAMERA_PAN_RATE: f32 = 1.0;

    let (mut camera_transform, action_state) = query.single_mut();

    let activate_pan = action_state.pressed(&CameraMovement::ActivatePan);
    if activate_pan {
        let camera_pan_vector = action_state.axis_pair(&CameraMovement::Pan);
        camera_transform.translation.x -= CAMERA_PAN_RATE * camera_pan_vector.x;
        camera_transform.translation.y += CAMERA_PAN_RATE * camera_pan_vector.y;
    }
}

fn zoom(
    mut query: Query<(&mut OrthographicProjection, &ActionState<CameraMovement>), With<Camera2d>>,
) {
    const CAMERA_ZOOM_RATE: f32 = 0.05;

    let (mut camera_projection, action_state) = query.single_mut();
    let zoom_delta = action_state.value(&CameraMovement::Zoom);

    camera_projection.scale *= 1. - zoom_delta * CAMERA_ZOOM_RATE;
}

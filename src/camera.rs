use bevy::{color::palettes::css::MINT_CREAM, prelude::*};
use leafwing_input_manager::prelude::*;

use crate::canvas::SIZE;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<CameraMovement>::default())
        .insert_resource(ClashStrategy::PrioritizeLongest)
        .add_systems(Startup, setup)
        .add_systems(Update, (pan, zoom, zoom_scroll));
}

pub fn setup(mut commands: Commands) {
    use CameraMovement::*;

    let input_map = InputMap::default()
        .with(ZoomModifier, ModifierKey::Alt)
        .with_dual_axis(
            Pan,
            DualAxislikeChord::new(MouseButton::Right, MouseMove::default()),
        )
        .with_axis(
            Zoom,
            AxislikeChord::new(
                ButtonlikeChord::from_single(MouseButton::Right).with(ModifierKey::Alt),
                MouseMoveAxis::Y,
            ),
        )
        .with_axis(ZoomWheel, MouseScrollAxis::Y);

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
    Pan,
    Zoom,
    ZoomModifier,
    ZoomWheel,
}

impl Actionlike for CameraMovement {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            CameraMovement::Pan => InputControlKind::DualAxis,
            CameraMovement::Zoom => InputControlKind::Axis,
            CameraMovement::ZoomModifier => InputControlKind::Button,
            CameraMovement::ZoomWheel => InputControlKind::Axis,
        }
    }
}

fn pan(
    mut query: Query<
        (
            &mut Transform,
            &OrthographicProjection,
            &ActionState<CameraMovement>,
        ),
        With<Camera2d>,
    >,
) {
    let (mut camera_transform, camera_projection, action_state) = query.single_mut();
    let camera_pan_vector = action_state.axis_pair(&CameraMovement::Pan);
    camera_transform.translation.x -= camera_projection.scale * camera_pan_vector.x;
    camera_transform.translation.y += camera_projection.scale * camera_pan_vector.y;
}

fn zoom(
    mut query: Query<(&mut OrthographicProjection, &ActionState<CameraMovement>), With<Camera2d>>,
) {
    const CAMERA_ZOOM_RATE: f32 = 0.005;
    let (mut camera_projection, action_state) = query.single_mut();

    if let Some(mouse_y) = action_state.axis_data(&CameraMovement::Zoom) {
        camera_projection.scale *= 1.0 - (mouse_y.value * CAMERA_ZOOM_RATE);
    }
}

const MIN_ZOOM: f32 = 1. / 16.;
const MAX_ZOOM: f32 = 8.0;

fn zoom_scroll(
    mut query: Query<(&mut OrthographicProjection, &ActionState<CameraMovement>), With<Camera2d>>,
) {
    let (mut proj, action_state) = query.single_mut();
    if let Some(data) = action_state.axis_data(&CameraMovement::ZoomWheel) {
        let new_scale = match proj.scale {
            x if x < 1.0 => snap_zoom_level(x, 16.0, data.value),
            x if x >= 1.0 && x < 4.0 => snap_zoom_level(x, 8.0, data.value),
            x => snap_zoom_level(x, 4.0, data.value),
        };

        proj.scale = new_scale.clamp(MIN_ZOOM, MAX_ZOOM);
    }
}

fn snap_zoom_level(scale: f32, increment: f32, steps: f32) -> f32 {
    ((scale + (-steps / increment)) * increment).round() / increment
}

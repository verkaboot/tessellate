use bevy::{input::mouse::MouseWheel, prelude::*};
use leafwing_input_manager::prelude::*;

use crate::canvas::SIZE;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<CameraMovement>::default())
        .insert_resource(ClashStrategy::PrioritizeLongest)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                pan.run_if(not(is_zooming)),
                zoom,
                zoom_scroll.run_if(on_event::<MouseWheel>()),
            ),
        );
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

fn is_zooming(query: Query<&ActionState<CameraMovement>, With<Camera2d>>) -> bool {
    let action_state = query.single();
    action_state.pressed(&CameraMovement::ZoomModifier)
}

fn zoom(
    mut query: Query<(&mut OrthographicProjection, &ActionState<CameraMovement>), With<Camera2d>>,
) {
    const CAMERA_ZOOM_RATE: f32 = -0.005;
    let (mut camera_projection, action_state) = query.single_mut();

    if let Some(mouse_y) = action_state.axis_data(&CameraMovement::Zoom) {
        camera_projection.scale = (camera_projection.scale
            * (1.0 - (mouse_y.value * CAMERA_ZOOM_RATE)))
            .clamp(MIN_ZOOM, MAX_ZOOM);
    }
}

const MIN_ZOOM: f32 = 1. / 16.;
const MAX_ZOOM: f32 = 8.0;

fn zoom_scroll(
    mut query: Query<(&mut OrthographicProjection, &ActionState<CameraMovement>), With<Camera2d>>,
) {
    let (mut camera_projection, action_state) = query.single_mut();
    if let Some(mouse_wheel_data) = action_state.axis_data(&CameraMovement::ZoomWheel) {
        camera_projection.scale = calculate_zoom(camera_projection.scale, mouse_wheel_data.value)
            .clamp(MIN_ZOOM, MAX_ZOOM);
    }
}

fn calculate_zoom(camera_projection_scale: f32, scroll_amount: f32) -> f32 {
    let new_scale = match camera_projection_scale {
        x if x < 1.0 => snap_zoom_level(x, 16.0, scroll_amount),
        x if x >= 1.0 && x < 4.0 => snap_zoom_level(x, 8.0, scroll_amount),
        x => snap_zoom_level(x, 4.0, scroll_amount),
    };
    new_scale
}

fn snap_zoom_level(scale: f32, snap_increments: f32, scroll_amount: f32) -> f32 {
    ((scale + (-scroll_amount / snap_increments)) * snap_increments).round() / snap_increments
}

use bevy::{input::mouse::MouseWheel, prelude::*, utils};

use canvas::{tool::ToolData, SIZE};
use error::Result;
use leafwing_input_manager::prelude::ActionState;

use crate::{controls, msg::TerrainCanvasDragged};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, pan.map(utils::warn));
}

#[derive(Component)]
pub struct MainCamera;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Transform::from_translation(Vec3::new(SIZE.x as f32 / 2.0, SIZE.y as f32 / 2.0, 0.0)),
        MainCamera,
        IsDefaultUiCamera,
    ));
}

pub fn pan(
    mut event: EventReader<TerrainCanvasDragged>,
    mut camera_q: Query<(&mut Transform, &OrthographicProjection), With<MainCamera>>,
    action_q: Query<&ActionState<input::Action>>,
) -> Result<()> {
    for e in event.read() {
        let action_state = action_q.get_single()?;
        if !action_state.pressed(&input::Action::PanCamera) {
            return Ok(());
        }

        let (mut camera_transform, camera_projection) = camera_q.get_single_mut()?;
        let delta = e.0.delta;
        camera_transform.translation.x -= camera_projection.scale * delta.x;
        camera_transform.translation.y -= camera_projection.scale * -delta.y;
    }

    Ok(())
}

fn zoom(
    mut query: Query<&mut OrthographicProjection, With<Camera2d>>,
    mouse_data: Res<ToolData>,
) -> Result<()> {
    const CAMERA_ZOOM_RATE: f32 = -0.005;
    let mut camera_projection = query.get_single_mut()?;
    let delta_y = mouse_data.screen_delta().y;
    camera_projection.scale =
        (camera_projection.scale * (1.0 - (delta_y * CAMERA_ZOOM_RATE))).clamp(MIN_ZOOM, MAX_ZOOM);

    Ok(())
}

const MIN_ZOOM: f32 = 1. / 16.;
const MAX_ZOOM: f32 = 8.0;

fn zoom_scroll(
    mut query: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut mouse_wheel_event: EventReader<MouseWheel>,
) -> Result<()> {
    for mouse_scroll in mouse_wheel_event.read() {
        let mut camera_projection = query.get_single_mut()?;
        camera_projection.scale =
            calculate_zoom(camera_projection.scale, mouse_scroll.y).clamp(MIN_ZOOM, MAX_ZOOM);
    }

    Ok(())
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

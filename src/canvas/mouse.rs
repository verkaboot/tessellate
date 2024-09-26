use bevy::{prelude::*, render::extract_resource::ExtractResource, window::PrimaryWindow};

use crate::error::{Error, Result};

#[derive(Resource, Clone, Copy, ExtractResource)]
pub struct MouseData {
    pub left_button_pressed: bool,
    pub world_pos: MousePositions,
    pub screen_pos: MousePositions,
}

impl MouseData {
    pub fn screen_delta(&self) -> Vec2 {
        self.screen_pos[0] - self.screen_pos[1]
    }
}
pub type MousePositions = [Vec2; 4];

pub fn setup(mut commands: Commands) {
    commands.insert_resource(MouseData {
        left_button_pressed: false,
        world_pos: [Vec2::ZERO; 4],
        screen_pos: [Vec2::ZERO; 4],
    })
}

pub fn update_position(
    mut m: ResMut<MouseData>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
) -> Result<()> {
    let (camera, camera_transform) = q_camera.single();

    let window = q_window.get_single()?;

    let screen_pos = window
        .cursor_position()
        .ok_or(Error::Custom("Cursor not found in window".to_owned()))?;
    let world_pos = camera
        .viewport_to_world_2d(camera_transform, screen_pos)
        .ok_or(Error::Custom(
            "Unable to get world position of cursor".to_owned(),
        ))?;
    m.world_pos = [world_pos, m.world_pos[0], m.world_pos[1], m.world_pos[2]];
    m.screen_pos = [
        screen_pos,
        m.screen_pos[0],
        m.screen_pos[1],
        m.screen_pos[2],
    ];

    Ok(())
}

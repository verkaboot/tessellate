use bevy::{prelude::*, render::extract_resource::ExtractResource, window::PrimaryWindow};

use error::{Error, Result};

#[derive(Resource, Debug, Default, Reflect, Clone, Copy, ExtractResource)]
pub struct ToolData {
    pub tool_type: ToolType,
    pub tool_active: bool,
    pub world_pos: MousePositions,
    pub screen_pos: MousePositions,
}

#[derive(Default, Debug, Reflect, Clone, Copy)]
pub enum ToolType {
    #[default]
    Select,
    Paint,
    Erase,
}

impl ToolData {
    pub fn screen_delta(&self) -> Vec2 {
        self.screen_pos[0] - self.screen_pos[1]
    }
}
pub type MousePositions = [Vec2; 4];

pub fn setup(mut commands: Commands) {
    commands.insert_resource(ToolData::default());
}

pub fn update_position(
    mut m: ResMut<ToolData>,
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

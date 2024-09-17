use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource, render_asset::RenderAssetUsages, render_resource::*,
    },
    window::PrimaryWindow,
};

use super::SIZE;
use crate::error::{Error, Result};

#[derive(Component)]
pub struct CanvasSprite;

#[derive(Resource, Clone, ExtractResource)]
pub struct CanvasImages {
    pub texture: Handle<Image>,
}

#[derive(Resource, Clone, Copy, ExtractResource)]
pub struct MouseData {
    pub left_button_pressed: bool,
    pub world_pos: MousePositions,
    pub screen_pos: MousePositions,
}

impl MouseData {
    pub fn world_delta(&self) -> Vec2 {
        self.world_pos[0] - self.world_pos[1]
    }
    pub fn screen_delta(&self) -> Vec2 {
        self.screen_pos[0] - self.screen_pos[1]
    }
}

pub type MousePositions = [Vec2; 4];

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[255, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let image_handle = images.add(image);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                flip_y: true,
                custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                SIZE.0 as f32 / 2.0,
                SIZE.1 as f32 / 2.0,
                0.0,
            )),
            texture: image_handle.clone(),
            ..default()
        },
        CanvasSprite,
    ));

    commands.insert_resource(CanvasImages {
        texture: image_handle,
    });

    commands.insert_resource(MouseData {
        left_button_pressed: false,
        world_pos: [Vec2::ZERO; 4],
        screen_pos: [Vec2::ZERO; 4],
    })
}

pub fn update_mouse_position(
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
            "Unable do get world position of cursor".to_owned(),
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

pub fn update_mouse_button_state(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_position: ResMut<MouseData>,
) {
    mouse_position.left_button_pressed = mouse_button_input.pressed(MouseButton::Left);
}

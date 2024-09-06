use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource, render_asset::RenderAssetUsages, render_resource::*,
    },
    window::PrimaryWindow,
};

use super::SIZE;

#[derive(Component)]
pub struct CanvasSprite;

#[derive(Resource, Clone, ExtractResource)]
pub struct CanvasImages {
    pub texture: Handle<Image>,
}

#[derive(Resource, Clone, Copy, ExtractResource)]
pub struct MouseData {
    pub left_button_pressed: bool,
    pub pos: MousePositions,
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
        pos: [Vec2::ZERO; 4],
    })
}

pub fn update_mouse_position(
    mut m: ResMut<MouseData>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();

    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        m.pos = [world_position, m.pos[0], m.pos[1], m.pos[2]];
    }
}

pub fn update_mouse_button_state(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_position: ResMut<MouseData>,
) {
    mouse_position.left_button_pressed = mouse_button_input.pressed(MouseButton::Left);
}

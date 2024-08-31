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
    pub texture_a: Handle<Image>,
    pub texture_b: Handle<Image>,
}

#[derive(Resource, Clone, Copy, ExtractResource)]
pub struct MousePosition {
    pub position: Vec2,
}

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let image0 = images.add(image.clone());
    let image1 = images.add(image);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
                ..default()
            },
            texture: image0.clone(),
            ..default()
        },
        CanvasSprite,
    ));

    commands.insert_resource(CanvasImages {
        texture_a: image0,
        texture_b: image1,
    });

    commands.insert_resource(MousePosition {
        position: Vec2 { x: 1.0, y: 1.0 },
    })
}

pub fn switch_textures(
    canvas_texture: Res<CanvasImages>,
    mut current_texture_q: Query<&mut Handle<Image>, With<CanvasSprite>>,
) {
    let mut displayed = current_texture_q.single_mut();
    if *displayed == canvas_texture.texture_a {
        *displayed = canvas_texture.texture_b.clone_weak();
    } else {
        *displayed = canvas_texture.texture_a.clone_weak();
    }
}

pub fn update_mouse_position(
    mut mouse_position: ResMut<MousePosition>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();

    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        mouse_position.position = world_position;
    }
}

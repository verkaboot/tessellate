use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource, render_asset::RenderAssetUsages, render_resource::*,
    },
};

use super::SIZE;

#[derive(Component)]
pub struct CanvasSprite;

#[derive(Resource, Clone, ExtractResource)]
pub struct CanvasImages {
    pub texture: Handle<Image>,
}

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
}

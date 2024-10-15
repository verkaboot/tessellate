use bevy::{
    prelude::*,
    render::{
        extract_component::ExtractComponent, extract_resource::ExtractResource,
        render_asset::RenderAssetUsages, render_resource::*,
    },
    sprite::Anchor,
};

use super::SIZE;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CanvasSprite>()
        .add_systems(Update, update_canvas_sprite);
}

#[derive(Resource, Clone, ExtractResource)]
pub struct CanvasImages {
    pub layered_texture: Handle<Image>,
    pub sprite_image: Handle<Image>,
    pub active_layer: u32,
}

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut layered_texture = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 3,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );
    layered_texture.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let layered_texture_handle = images.add(layered_texture);

    let mut sprite_image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );
    sprite_image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let sprite_image_handle = images.add(sprite_image);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                flip_y: true,
                custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
                anchor: Anchor::BottomLeft,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(SIZE.0 as f32, SIZE.1 as f32, 0.0)),
            texture: sprite_image_handle.clone(),
            ..default()
        },
        CanvasSprite::default(),
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                flip_y: true,
                custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
                anchor: Anchor::BottomLeft,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(SIZE.0 as f32, 0.0, 0.0)),
            texture: sprite_image_handle.clone(),
            ..default()
        },
        CanvasSprite::default(),
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                flip_y: true,
                custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
                anchor: Anchor::BottomLeft,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, SIZE.1 as f32, 0.0)),
            texture: sprite_image_handle.clone(),
            ..default()
        },
        CanvasSprite::default(),
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                flip_y: true,
                custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
                anchor: Anchor::BottomLeft,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            texture: sprite_image_handle.clone(),
            ..default()
        },
        CanvasSprite::default(),
    ));

    commands.insert_resource(CanvasImages {
        layered_texture: layered_texture_handle,
        sprite_image: sprite_image_handle,
        active_layer: 0,
    });
}

#[derive(Component, ExtractComponent, DerefMut, Deref, Clone, Copy, Debug, Default, Reflect)]
pub struct CanvasSprite(pub Vec2);

fn update_canvas_sprite(
    mut canvas_sprite_q: Query<(&mut CanvasSprite, &GlobalTransform), Changed<GlobalTransform>>,
) {
    for (mut canvas_sprite, global_transform) in &mut canvas_sprite_q {
        *canvas_sprite = CanvasSprite(global_transform.translation().xy());
    }
}

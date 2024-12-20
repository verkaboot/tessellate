use bevy::{
    prelude::*,
    render::{render_asset::RenderAssetUsages, render_resource::*},
    sprite::Anchor,
};

use grid::Grid;

use canvas::{
    bind_groups::{CanvasImages, CanvasSprite},
    SIZE,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreStartup, setup)
        .add_systems(Update, update_sprite_position_for_gpu);
}

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut layered_texture = Image::new_fill(
        Extent3d {
            width: SIZE.x,
            height: SIZE.y,
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
            width: SIZE.x,
            height: SIZE.y,
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
        Sprite {
            image: sprite_image_handle.clone(),
            flip_y: true,
            custom_size: Some(Vec2::new(SIZE.x as f32, SIZE.y as f32)),
            anchor: Anchor::BottomLeft,
            ..default()
        },
        Transform::from_translation(Vec3::new(SIZE.x as f32, SIZE.y as f32, 0.0)),
        CanvasSprite::default(),
    ));

    commands.spawn((
        Sprite {
            image: sprite_image_handle.clone(),
            flip_y: true,
            custom_size: Some(Vec2::new(SIZE.x as f32, SIZE.y as f32)),
            anchor: Anchor::BottomLeft,
            ..default()
        },
        Transform::from_translation(Vec3::new(SIZE.x as f32, 0.0, 0.0)),
        CanvasSprite::default(),
    ));

    commands.spawn((
        Sprite {
            image: sprite_image_handle.clone(),
            flip_y: true,
            custom_size: Some(Vec2::new(SIZE.x as f32, SIZE.y as f32)),
            anchor: Anchor::BottomLeft,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, SIZE.y as f32, 0.0)),
        CanvasSprite::default(),
    ));

    commands
        .spawn((
            Sprite {
                image: sprite_image_handle.clone(),
                flip_y: true,
                custom_size: Some(SIZE.as_vec2()),
                anchor: Anchor::BottomLeft,
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            CanvasSprite::default(),
        ))
        .with_child(Text2d::new("Sprite"));

    commands.insert_resource(CanvasImages {
        layered_texture: layered_texture_handle,
        sprite_image: sprite_image_handle,
        active_layer: 0,
    });
}

fn update_sprite_position_for_gpu(
    mut canvas_sprite_q: Query<(&mut CanvasSprite, &GlobalTransform), Changed<GlobalTransform>>,
) {
    for (mut canvas_sprite, global_transform) in &mut canvas_sprite_q {
        *canvas_sprite = CanvasSprite(global_transform.translation().xy());
    }
}

pub struct ArtTile {
    image: Handle<Image>,
}

fn match_sprites_to_grid(event: EventReader<event::terrain::Updated>, grid: Res<Grid<ArtTile>>) {
    if grid.is_changed() {}
}

// Add an event for when terrain is created or removed,
// then read that event to make changes to the canvas sprites.

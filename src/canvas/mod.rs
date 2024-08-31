mod compute;

use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource, render_asset::RenderAssetUsages, render_resource::*,
    },
};
use compute::GameOfLifeComputePlugin;

pub const SIZE: (u32, u32) = (1280, 720);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(GameOfLifeComputePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, switch_textures);
}

#[derive(Component)]
struct GameOfLifeSprite;

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
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
        GameOfLifeSprite,
    ));

    commands.insert_resource(GameOfLifeImages {
        texture_a: image0,
        texture_b: image1,
    });
}

// Switch texture to display every frame to show the one that was written to most recently.
fn switch_textures(
    images: Res<GameOfLifeImages>,
    mut displayed: Query<&mut Handle<Image>, With<GameOfLifeSprite>>,
) {
    let mut displayed = displayed.single_mut();
    if *displayed == images.texture_a {
        *displayed = images.texture_b.clone_weak();
    } else {
        *displayed = images.texture_a.clone_weak();
    }
}

#[derive(Resource, Clone, ExtractResource)]
pub struct GameOfLifeImages {
    texture_a: Handle<Image>,
    texture_b: Handle<Image>,
}

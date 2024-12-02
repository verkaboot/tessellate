use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        view::RenderLayers,
    },
    utils,
    window::WindowResized,
};

use error::Result;
use ui::theme::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(Color::NONE))
        .add_systems(
            Startup,
            (checkered_background.map(utils::warn), (background_camera)),
        )
        .add_systems(Update, on_window_resize.map(utils::warn));
}

#[derive(Component)]
pub struct BackgroundImage;

fn checkered_background(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    windows: Query<&Window>,
) -> Result<()> {
    let window = windows.get_single()?;
    let size: UVec2 = window.physical_size();

    let image = generate_background_image();
    let image_handle = images.add(image);

    commands.spawn((
        Name::new("BackgroundImage"),
        Sprite {
            custom_size: Some(Vec2 {
                x: size.x as f32,
                y: size.y as f32,
            }),
            image: image_handle.clone(),
            ..default()
        },
        SliceScaleMode::Tile {
            stretch_value: 128.0,
        },
        RenderLayers::from_layers(&[1]),
        BackgroundImage,
    ));

    Ok(())
}

fn dark() -> [u8; 4] {
    TRANSPARENCY_BLOCKS.to_srgba().to_u8_array()
}

fn light() -> [u8; 4] {
    TRANSPARENCY_BLOCKS.lighter(0.02).to_srgba().to_u8_array()
}

fn generate_background_image() -> Image {
    Image::new(
        Extent3d {
            width: 2,
            height: 2,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        [light(), dark(), dark(), light()].as_flattened().to_vec(),
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    )
}

#[derive(Component)]
pub struct BackgroundCamera;

fn background_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("BackgroundCamera"),
        Camera2d,
        Camera {
            order: -1,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.9, 0.9, 0.9)),
            ..default()
        },
        RenderLayers::from_layers(&[1]),
        BackgroundCamera,
    ));
}

fn on_window_resize(
    mut window_resize_evr: EventReader<WindowResized>,
    windows: Query<&Window>,
    mut background_q: Query<(Entity, &mut Sprite), With<BackgroundImage>>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) -> Result<()> {
    for e in window_resize_evr.read() {
        let image = generate_background_image();
        let image_handle = images.add(image);
        let window = windows.get(e.window)?;
        let size: UVec2 = window.physical_size();
        let (entity, mut sprite) = background_q.get_single_mut()?;
        sprite.custom_size = Some(Vec2::new(size.x as f32, size.y as f32));
        commands.entity(entity).insert(image_handle);
    }

    Ok(())
}

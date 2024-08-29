use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("DrawableSpriteTexture"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[TextureFormat::Rgba8UnormSrgb],
        },
        ..default()
    };

    image.resize(size);
    image.data = vec![100; (size.width * size.height * 4) as usize];

    let image_handle = images.add(image);

    commands
        .spawn((
            Name::new("DrawableSprite"),
            SpriteBundle {
                texture: image_handle.clone(),
                ..default()
            },
        ))
        .observe(pointer_down);
}

#[derive(Event)]
pub struct OnDraw;

fn trigger_on_draw(mut commands: Commands) {}

fn pointer_down(trigger: Trigger<OnDraw>) {}

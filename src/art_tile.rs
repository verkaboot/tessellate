use bevy::{
    prelude::*,
    render::{render_asset::RenderAssetUsages, render_resource::*},
    sprite::Anchor,
};

use grid::{Grid, GridSettings};

use canvas::{
    bind_groups::{CanvasImage, CanvasSprite},
    SIZE,
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Grid::<ArtTile>::new(GridSettings {
        cell_size: SIZE,
        offset: SIZE.as_vec2() / 2.0,
    }));

    app.add_systems(PreStartup, setup_canvas_textures)
        .add_systems(Update, update_sprite_position_for_gpu)
        .add_systems(Update, match_sprites_to_grid);
}

pub fn setup_canvas_textures(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
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

    let mut composite_image = Image::new_fill(
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
    composite_image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let composite_view_handle = images.add(composite_image);

    commands
        .spawn((
            Sprite {
                image: composite_view_handle.clone(),
                flip_y: true,
                custom_size: Some(SIZE.as_vec2()),
                anchor: Anchor::BottomLeft,
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            CanvasSprite::default(),
        ))
        .with_child(Text2d::new("Sprite"));

    commands.insert_resource(CanvasImage {
        layered_texture: layered_texture_handle,
        composite_view: composite_view_handle,
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

fn match_sprites_to_grid(
    mut event: EventReader<event::terrain::Updated>,
    grid: Res<Grid<ArtTile>>,
) {
    for event in event.read() {
        println!("{:?}", event.coord);
    }
}

/*

Need to decide whether it's worth it to start out with
texture atlases, and you are drawing on parts of the atlas.
I think it is worth it to start out this way, because it
actually simplifies keeping track of the art, and I think
it might actually help the artists to budget their tiles better
by thinking about them in terms of what is going to be shown
on the screen at the same time.

So, we will need to init the atlas, and then every time a
sprite is created, it references a map with the neighbor
contraints and an atlas index that matches that constraint.

*/

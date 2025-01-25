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

use crate::terrain::TerrainType;

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

    // TODO: The app crashes without an initial sprite. Figure out a way to prevent that.
    commands
        .spawn((
            Sprite {
                image: composite_view_handle.clone(),
                flip_y: true,
                custom_size: Some(SIZE.as_vec2()),
                anchor: Anchor::BottomLeft,
                ..default()
            },
            Transform::from_translation(Vec3::new(-100000.0, 0.0, 0.0)),
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

pub struct ArtTile;
// image: Handle<Image>,

fn match_sprites_to_grid(
    mut commands: Commands,
    mut event: EventReader<event::terrain::Updated>,
    mut art_grid: ResMut<Grid<ArtTile>>,
    terrain_grid: Res<Grid<TerrainType>>,
    canvas_image: Res<CanvasImage>,
) {
    for event in event.read() {
        let affected_tiles = event.terrain_coord.inverse_corners();
        for tile_coord in affected_tiles {
            let terrain_data = tile_coord.corners().map(|coord| terrain_grid.get(&coord));
            if let Some(data) = art_grid.get(&tile_coord) {
                if let Some(entity_commands) = commands.get_entity(data.entity) {
                    entity_commands.despawn_recursive();
                }
            }
            if terrain_data.iter().any(|cell| cell.is_some()) {
                let entity = commands
                    .spawn((
                        Sprite {
                            image: canvas_image.composite_view.clone(),
                            flip_y: true,
                            custom_size: Some(SIZE.as_vec2()),
                            anchor: Anchor::BottomLeft,
                            ..default()
                        },
                        Transform::from_translation(
                            tile_coord.to_world_pos(art_grid.settings).extend(0.0),
                        ),
                        CanvasSprite::default(),
                    ))
                    // Debug coord text
                    .with_child((
                        Text2d::new(format!("{}", tile_coord)),
                        Transform::from_xyz(
                            art_grid.settings.cell_size.x as f32 / 2.0,
                            art_grid.settings.cell_size.y as f32 / 2.0,
                            1.0,
                        ),
                    ))
                    .id();
                art_grid.insert(tile_coord, entity, ArtTile);
            }
        }
    }
}

/*

Need to decide whether it's worth it to start out with
texture atlases, where you would be drawing on parts of the atlas.
I think it is worth it to start out this way, because it
actually simplifies keeping track of the art, and I think
it might actually help the artists to budget their tiles better
by thinking about them in terms of what is going to be shown
on the screen at the same time.

So, we will need to init the atlas, and then every time a
sprite is created, it references a map with the neighbor
contraints and an atlas index that matches that constraint.

*/

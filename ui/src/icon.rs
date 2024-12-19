use bevy::{ecs::system::EntityCommand, prelude::*, ui::Val::*};

pub enum Icon {
    Brush,
    Eraser,
    Layer,
    ColorPicker,
    PaintView,
    TerrainView,
}

impl EntityCommand for Icon {
    fn apply(self, entity: Entity, world: &mut World) {
        let asset_server = world.resource::<AssetServer>();
        let image_handle: Handle<Image> = match self {
            Icon::Brush => asset_server.load("icons/brush.png"),
            Icon::Eraser => asset_server.load("icons/eraser.png"),
            Icon::Layer => asset_server.load("icons/layer.png"),
            Icon::ColorPicker => asset_server.load("icons/color_picker.png"),
            Icon::PaintView => asset_server.load("icons/paint_view.png"),
            Icon::TerrainView => asset_server.load("icons/terrain_view.png"),
        };
        let icon = world
            .spawn((
                Name::new("Icon"),
                ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Px(42.0),
                        height: Px(42.0),
                        ..default()
                    },
                    image: UiImage {
                        texture: image_handle,
                        ..default()
                    },
                    ..default()
                },
            ))
            .id();
        world.entity_mut(entity).add_child(icon);
    }
}

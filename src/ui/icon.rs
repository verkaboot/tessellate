use bevy::{ecs::system::EntityCommand, prelude::*, ui::Val::*};

pub enum Icon {
    Brush,
}

impl EntityCommand for Icon {
    fn apply(self, entity: Entity, world: &mut World) {
        let asset_server = world.resource::<AssetServer>();
        let image_handle: Handle<Image> = match self {
            Icon::Brush => asset_server.load("icons/brush.png"),
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

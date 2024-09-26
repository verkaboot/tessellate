use bevy::{ecs::system::EntityCommand, prelude::*};

pub enum Icon {
    Brush,
}

impl EntityCommand for Icon {
    fn apply(self, entity: Entity, world: &mut World) {
        let asset_server = world.resource::<AssetServer>();
        let image_handle: Handle<Image> = match self {
            Icon::Brush => asset_server.load("icons/brush.png"),
        };
        world.entity_mut(entity).insert(UiImage {
            texture: image_handle,
            ..default()
        });
    }
}

use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use super::Spawn;

pub trait FlexWidget {
    fn flex(&mut self) -> EntityCommands;
}

impl<T: Spawn> FlexWidget for T {
    fn flex(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("Flex"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
    }
}

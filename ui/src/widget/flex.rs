use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use super::Spawn;

pub trait FlexWidget {
    fn flex_row(&mut self) -> EntityCommands;
    fn flex_col(&mut self) -> EntityCommands;
}

impl<T: Spawn> FlexWidget for T {
    fn flex_row(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("Flex Row"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
        ))
    }
    fn flex_col(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("Flex Col"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
    }
}

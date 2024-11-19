use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::theme::{self};

use super::Spawn;

pub trait TextWidget {
    fn text(&mut self, string: &str) -> EntityCommands;
}

impl<T: Spawn> TextWidget for T {
    fn text(&mut self, string: &str) -> EntityCommands {
        self.spawn((
            Name::new("Text"),
            TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: string.to_string(),
                        style: TextStyle {
                            font_size: 24.0,
                            color: theme::TEXT,
                            ..default()
                        },
                    }],
                    ..default()
                },
                style: Style {
                    display: Display::Block,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
    }
}

use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    ui::{RelativeCursorPosition, Val::*},
};

use super::Spawn;

pub trait CanvasWidget {
    fn canvas(&mut self) -> EntityCommands;
}

impl<T: Spawn> CanvasWidget for T {
    fn canvas(&mut self) -> EntityCommands {
        self.ui_spawn((
            Name::new("Canvas"),
            Node {
                width: Percent(100.0),
                height: Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
            Interaction::default(),
            RelativeCursorPosition::default(),
        ))
    }
}

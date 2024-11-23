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
        self.spawn((
            Name::new("Canvas"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                ..default()
            },
            Interaction::default(),
            RelativeCursorPosition::default(),
        ))
    }
}

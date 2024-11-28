use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use super::Spawn;

pub trait ListWidget {
    fn list(&mut self) -> EntityCommands;
}

impl<T: Spawn> ListWidget for T {
    fn list(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("List"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Auto,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
    }
}

pub trait SelectList<T> {
    fn new(item: T) -> Self;
    fn get_selected(&self) -> &T;
    fn select(&mut self, index: usize);
}

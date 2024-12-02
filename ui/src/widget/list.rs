use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use super::Spawn;

pub trait ListWidget {
    fn list(&mut self) -> EntityCommands;
}

impl<T: Spawn> ListWidget for T {
    fn list(&mut self) -> EntityCommands {
        self.ui_spawn((
            Name::new("List"),
            Node {
                width: Percent(100.0),
                height: Auto,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
    }
}

pub trait SelectList {
    type Item;
    fn new(item: Self::Item) -> Self;
    fn get_selected(&self) -> &Self::Item;
    fn select(&mut self, index: usize);
}

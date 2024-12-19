use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};
use input::interaction::InteractionPalette;

use crate::theme::BUTTON_BACKGROUND;

use super::Spawn;

pub trait ButtonWidget {
    fn button(&mut self) -> EntityCommands;
}

impl<T: Spawn> ButtonWidget for T {
    fn button(&mut self) -> EntityCommands {
        self.ui_spawn((
            Name::new("ButtonParent"),
            Button,
            Node {
                display: Display::Block,
                width: Px(42.0),
                height: Px(42.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BUTTON_BACKGROUND),
            BorderRadius::all(Px(7.5)),
            InteractionPalette::default(BUTTON_BACKGROUND),
        ))
    }
}

use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use crate::ui::{interaction::InteractionPalette, theme::*};

use super::Spawn;

pub trait ButtonWidget {
    fn button(&mut self) -> EntityCommands;
}

impl<T: Spawn> ButtonWidget for T {
    fn button(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("ButtonParent"),
            ButtonBundle {
                style: Style {
                    display: Display::Block,
                    width: Px(42.0),
                    height: Px(42.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(BUTTON_BACKGROUND),
                border_radius: BorderRadius::all(Px(7.5)),
                ..default()
            },
            InteractionPalette::default(BUTTON_BACKGROUND),
        ))
    }
}

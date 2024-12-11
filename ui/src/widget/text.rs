use bevy::{ecs::system::EntityCommands, prelude::*};

use input::trigger::{OnResourceUpdated, WatchResource};

use super::Spawn;
use crate::theme::{self};

pub trait TextValue: Resource + Copy + Clone + std::fmt::Debug + std::fmt::Display {}

pub trait TextWidget {
    fn text<V: TextValue>(&mut self) -> EntityCommands;
}

impl<T: Spawn> TextWidget for T {
    fn text<V: TextValue>(&mut self) -> EntityCommands {
        let mut entity = self.ui_spawn((
            Name::new("Text"),
            Text("-".into()),
            TextFont {
                font_size: 22.0,
                ..default()
            },
            TextColor(theme::TEXT),
            WatchResource::<V>::new(),
        ));

        entity.observe(update_text::<V>);

        entity
    }
}

fn update_text<V: TextValue>(
    trigger: Trigger<OnResourceUpdated<V>>,
    mut writer: TextUiWriter,
    resource: Res<V>,
) {
    *writer.text(trigger.entity(), 0) = format!("{}", (*resource).to_string());
}

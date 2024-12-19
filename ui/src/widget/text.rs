use bevy::{ecs::system::EntityCommands, prelude::*, utils};

use crate::{
    interaction::{OnResourceUpdated, WatchResource},
    theme::{self},
};
use error::Result;

use super::Spawn;

pub trait TextValue: Resource + Copy + Clone + std::fmt::Debug + std::fmt::Display {}

pub trait TextWidget {
    fn text<V: TextValue>(&mut self, label: &str) -> EntityCommands;
}

impl<T: Spawn> TextWidget for T {
    fn text<V: TextValue>(&mut self, label: &str) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Text"),
            TextBundle::from_sections([
                TextSection {
                    value: label.to_string(),
                    ..default()
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font_size: 24.0,
                        color: theme::TEXT,
                        ..default()
                    },
                },
            ]),
            WatchResource::<V>::new(),
        ));

        entity.observe(update_text::<V>.map(utils::warn));

        entity
    }
}

fn update_text<V: TextValue>(
    trigger: Trigger<OnResourceUpdated<V>>,
    resource: Res<V>,
    mut text_q: Query<&mut Text>,
) -> Result<()> {
    let mut text = text_q.get_mut(trigger.entity())?;
    text.sections[1].value = format!("{}", (*resource).to_string());
    Ok(())
}

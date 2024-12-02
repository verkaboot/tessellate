use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*, utils};

use crate::interaction::{OnResourceUpdated, WatchResource};
use error::Result;

use super::Spawn;

pub trait ListWidget {
    fn select_list<L: SelectList + Resource>(&mut self) -> EntityCommands;
}

#[derive(Component)]
pub struct SelectListParent;

impl<T: Spawn> ListWidget for T {
    fn select_list<L: SelectList + Resource>(&mut self) -> EntityCommands {
        let mut entity = self.spawn((
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
            WatchResource::<L>::new(),
            SelectListParent,
        ));

        entity.observe(update_children::<L>.map(utils::warn));

        entity
    }
}

pub trait SelectList {
    type Item: Spawn;
    fn new(item: Self::Item) -> Self;
    fn get_selected(&self) -> &Self::Item;
    fn select(&mut self, index: usize);
}

fn update_children<L: SelectList + Resource>(
    trigger: Trigger<OnResourceUpdated<L>>,
    list: Res<L>,
    list_q: Query<Entity, With<SelectListParent>>,
    mut commands: Commands,
) -> Result<()> {
    let x = list_q.get(trigger.entity())?;

    commands.entity(x).replace_children(&[]);

    Ok(())
}

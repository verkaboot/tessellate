use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use crate::interaction::OnPress;

pub trait Containers {
    fn ui_root<C: Component + std::fmt::Debug>(&mut self, marker_component: C) -> EntityCommands;
}

#[derive(Component)]
pub struct UiRoot;

impl Containers for Commands<'_, '_> {
    fn ui_root<C: Component + std::fmt::Debug>(&mut self, marker_component: C) -> EntityCommands {
        self.spawn((
            Name::new(format!("UI Root: {marker_component:?}")),
            marker_component,
            UiRoot,
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Start,
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
        ))
    }
}

pub fn set_root<C: Component>(
    _trigger: Trigger<OnPress>,
    mut root_q: Query<(&mut Visibility, Option<&C>), With<UiRoot>>,
) {
    for (mut visibility, c) in &mut root_q {
        match c {
            Some(_) => {
                *visibility = Visibility::Visible;
            }
            None => {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

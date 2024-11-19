use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use crate::interaction::OnPress;

pub trait Containers {
    fn ui_root<C: RootState>(&mut self, root_state: C) -> EntityCommands;
}

pub trait RootState: Component + std::fmt::Debug + Eq + PartialEq {}

impl Containers for Commands<'_, '_> {
    fn ui_root<C: RootState>(&mut self, root_state: C) -> EntityCommands {
        self.spawn((
            Name::new(format!("UI Root: {root_state:?}")),
            root_state,
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

pub fn set_root<C: RootState>(
    root_state: C,
) -> impl Fn(Trigger<OnPress>, Query<(&mut Visibility, &C)>) {
    move |_trigger: Trigger<OnPress>, mut root_q: Query<(&mut Visibility, &C)>| {
        for (mut visibility, c) in &mut root_q {
            if root_state == *c {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

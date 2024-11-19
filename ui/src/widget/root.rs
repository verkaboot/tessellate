use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use crate::interaction::OnPress;

pub trait Containers {
    fn ui_root<C: RootState>(&mut self, root_state: C) -> EntityCommands;
}

pub trait RootState: Component + std::fmt::Debug + Eq + PartialEq + Clone + Copy {}

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

#[derive(Resource)]
pub struct CurrentState<C: RootState>(pub C);

pub fn set_root<C: RootState>(
    root_state: C,
) -> impl Fn(Trigger<OnPress>, Query<(&mut Visibility, &C)>, ResMut<CurrentState<C>>) {
    move |_trigger: Trigger<OnPress>,
          mut root_q: Query<(&mut Visibility, &C)>,
          mut current_state: ResMut<CurrentState<C>>| {
        for (mut visibility, c) in &mut root_q {
            if root_state == *c {
                *visibility = Visibility::Visible;
                *current_state = CurrentState(*c);
                println!("{:?}", current_state.0);
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

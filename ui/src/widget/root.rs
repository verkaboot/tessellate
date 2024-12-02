use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use crate::interaction::OnPress;

use super::text::TextValue;

pub trait Containers {
    fn ui_root<C: RootState>(&mut self, root_state: C) -> EntityCommands;
}

pub trait RootState:
    Component + std::fmt::Debug + std::fmt::Display + Eq + PartialEq + Clone + Copy
{
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct CurrentState<C: RootState>(pub C);

impl<C: RootState> std::fmt::Display for CurrentState<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<C: RootState> TextValue for CurrentState<C> {}
impl Containers for Commands<'_, '_> {
    fn ui_root<C: RootState>(&mut self, root_state: C) -> EntityCommands {
        self.spawn((
            Name::new(format!("UI Root: {root_state:?}")),
            root_state,
            Node {
                width: Percent(100.0),
                height: Percent(100.0),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
    }
}

pub fn set_root<C: RootState>(root_state: C) -> impl Fn(Trigger<OnPress>, ResMut<CurrentState<C>>) {
    move |_trigger: Trigger<OnPress>, mut current_state: ResMut<CurrentState<C>>| {
        current_state.0 = root_state;
    }
}

pub fn watch_state<C: RootState>(
    mut vis_q: Query<(&mut Visibility, &C)>,
    current_state: Res<CurrentState<C>>,
) {
    if current_state.is_changed() {
        for (mut visibility, root_state) in &mut vis_q {
            if *root_state == current_state.0 {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

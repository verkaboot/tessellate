use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

pub trait Containers {
    fn ui_root<C: Component + std::fmt::Debug>(&mut self, component: C) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root<C: Component + std::fmt::Debug>(&mut self, marker_component: C) -> EntityCommands {
        self.spawn((
            Name::new(format!("UI Root: {marker_component:?}")),
            marker_component,
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

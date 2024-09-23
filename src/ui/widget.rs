use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use super::{
    icon::Icon,
    theme::{BUTTON_BACKGROUND, PANEL_BACKGROUND},
};

pub trait Widget {
    fn button(&mut self, icon: Icon) -> EntityCommands;

    fn top_bar(&mut self) -> EntityCommands;

    fn canvas(&mut self) -> EntityCommands;
}

impl<T: Spawn> Widget for T {
    fn button(&mut self, icon: Icon) -> EntityCommands {
        let entity = self.spawn((
            Name::new("Button"),
            ButtonBundle {
                style: Style {
                    width: Px(50.0),
                    height: Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(PANEL_BACKGROUND),
                ..default()
            },
        ));
        entity
    }

    fn top_bar(&mut self) -> EntityCommands {
        let entity = self.spawn((
            Name::new("Top Bar"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Px(75.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(BUTTON_BACKGROUND),
                ..default()
            },
        ));
        entity
    }

    fn canvas(&mut self) -> EntityCommands {
        let entity = self.spawn((
            Name::new("Canvas"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.8, 0.2, 0.1, 0.3)),
                ..default()
            },
        ));
        entity
    }
}

/// An extension trait for spawning UI containers.
pub trait Containers {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_root(&mut self) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
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

/// An internal trait for types that can spawn entities.
/// This is here so that [`Widgets`] can be implemented on all types that
/// are able to spawn entities.
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use super::{
    interaction::InteractionPalette,
    theme::{BUTTON_BACKGROUND, PANEL_BACKGROUND},
};

pub trait Widget {
    fn button(&mut self) -> EntityCommands;
    fn canvas(&mut self) -> EntityCommands;
    fn flex(&mut self) -> EntityCommands;
    fn panel(&mut self, direction: PanelDirection) -> EntityCommands;
}

pub enum PanelDirection {
    Wide,
    Tall,
}

const BORDER_RADIUS: BorderRadius = BorderRadius::new(Px(7.5), Px(32.0), Px(7.5), Px(32.0));

impl<T: Spawn> Widget for T {
    fn button(&mut self) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("ButtonParent"),
            NodeBundle {
                style: Style {
                    display: Display::Block,
                    width: Px(42.0),
                    height: Px(42.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ));

        entity.with_children(|parent| {
            parent.spawn((
                Name::new("Shadow"),
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Px(42.0),
                        height: Px(42.0),
                        top: Px(2.0),
                        left: Px(2.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba(0.013, 0.071, 0.107, 0.5)),
                    border_radius: BORDER_RADIUS,
                    ..default()
                },
            ));

            parent.spawn((
                Name::new("Button"),
                ButtonBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Px(42.0),
                        height: Px(42.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(BUTTON_BACKGROUND),
                    border_radius: BORDER_RADIUS,
                    ..default()
                },
                InteractionPalette::default(BUTTON_BACKGROUND),
            ));
        });

        entity
    }

    fn canvas(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("Canvas"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                ..default()
            },
            Interaction::default(),
        ))
    }

    fn flex(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("Flex"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
    }

    fn panel(&mut self, direction: PanelDirection) -> EntityCommands {
        let (width, height) = match direction {
            PanelDirection::Wide => (Percent(100.0), Px(75.0)),
            PanelDirection::Tall => (Px(75.0), Percent(100.0)),
        };
        self.spawn((
            Name::new("Panel"),
            NodeBundle {
                style: Style {
                    width,
                    height,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Px(8.0)),
                    ..default()
                },
                background_color: BackgroundColor(PANEL_BACKGROUND),
                ..default()
            },
        ))
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

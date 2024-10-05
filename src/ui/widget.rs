use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use super::{interaction::InteractionPalette, theme::*};

pub trait Widget {
    fn button(&mut self) -> EntityCommands;
    fn canvas(&mut self) -> EntityCommands;
    fn flex(&mut self) -> EntityCommands;
    fn panel(&mut self, direction: PanelDirection) -> EntityCommands;
    fn slider(&mut self, label: &str) -> EntityCommands;
}

pub enum PanelDirection {
    Wide,
    Tall,
}

impl<T: Spawn> Widget for T {
    fn button(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("ButtonParent"),
            ButtonBundle {
                style: Style {
                    display: Display::Block,
                    width: Px(42.0),
                    height: Px(42.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(BUTTON_BACKGROUND),
                border_radius: BorderRadius::all(Px(7.5)),
                ..default()
            },
            InteractionPalette::default(BUTTON_BACKGROUND),
        ))
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
        let (width, height, flex_direction) = match direction {
            PanelDirection::Wide => (Percent(100.0), Auto, FlexDirection::Row),
            PanelDirection::Tall => (Auto, Percent(100.0), FlexDirection::Column),
        };
        self.spawn((
            Name::new("Panel"),
            NodeBundle {
                style: Style {
                    width,
                    height,
                    flex_direction,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Px(4.0)),
                    margin: UiRect::px(0.0, 0.0, 1.0, 1.0),
                    border: UiRect::px(1.0, 0.0, 1.0, 0.0),
                    row_gap: Px(2.0),
                    column_gap: Px(2.0),
                    ..default()
                },
                border_color: BorderColor(PANEL_HIGHLIGHT),
                background_color: BackgroundColor(PANEL_BACKGROUND),
                ..default()
            },
            Outline::new(Px(1.0), Val::ZERO, PANEL_OUTLINE),
        ))
    }

    fn slider(&mut self, label: &str) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Slider"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::axes(Px(4.0), Px(2.0)),
                    min_width: Px(120.0),
                    height: Auto,
                    ..default()
                },
                background_color: BackgroundColor(SLIDER_BACKGROUND),
                border_radius: BorderRadius::all(Px(4.0)),
                ..default()
            },
        ));

        entity.with_children(|parent| {
            parent.spawn((
                Name::new("Label"),
                TextBundle::from_section(
                    label,
                    TextStyle {
                        font_size: 12.0,
                        color: TEXT,
                        ..default()
                    },
                ),
            ));

            parent
                .spawn((
                    Name::new("SliderSlot"),
                    NodeBundle {
                        style: Style {
                            width: Percent(100.0),
                            height: Px(4.0),
                            margin: UiRect::vertical(Px(4.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Start,
                            ..default()
                        },
                        background_color: BackgroundColor(SLIDER_SLOT),
                        border_radius: BorderRadius::all(Px(4.0)),
                        ..default()
                    },
                ))
                .with_children(|slot| {
                    slot.spawn((
                        Name::new("Slider Knob"),
                        NodeBundle {
                            style: Style {
                                width: Px(12.0),
                                height: Px(12.0),
                                border: UiRect::all(Px(0.5)),
                                ..default()
                            },
                            border_radius: BorderRadius::all(Percent(100.0)),
                            background_color: BackgroundColor(SLIDER_KNOB),
                            border_color: BorderColor(SLIDER_KNOB_OUTLINE),
                            ..default()
                        },
                    ));
                });
        });

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

use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use crate::ui::interaction::OnPress;
use crate::ui::theme::*;
use crate::ui::widget::Spawn;

pub trait SliderWidget {
    fn slider<R: Resource + std::fmt::Debug>(&mut self, label: &str) -> EntityCommands;
}

impl<T: Spawn> SliderWidget for T {
    fn slider<R: Resource + std::fmt::Debug>(&mut self, label: &str) -> EntityCommands {
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
                            padding: UiRect::right(Px(12.0)),
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
                        ButtonBundle {
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
                        SliderValue,
                    ))
                    .observe(
                        |_trigger: Trigger<OnPress>, resource: Res<R>| {
                            println!("Slider: {:?}", resource);
                        },
                    );
                });
        });

        entity
    }
}

#[derive(Component)]
pub struct SliderValue;
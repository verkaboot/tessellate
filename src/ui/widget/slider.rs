use bevy::ui::RelativeCursorPosition;
use bevy::utils;
use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};
use bevy_inspector_egui::egui::lerp;

use crate::error::{Error, Result};
use crate::ui::interaction::{OnDrag, OnResourceUpdated, OnUiNodeSizeChange, WatchResource};
use crate::ui::theme::*;
use crate::ui::widget::Spawn;

pub const PHI: f32 = 1.618;
pub const KNOB_HEIGHT: f32 = 14.0;
pub const KNOB_WIDTH: f32 = KNOB_HEIGHT * PHI;
pub const KNOB_PADDING: f32 = KNOB_WIDTH * (2.0 - PHI);

pub trait SliderWidget {
    fn slider<R: Resource + std::fmt::Debug + From<f32> + Into<f32> + Copy + Clone>(
        &mut self,
        label: &str,
    ) -> EntityCommands;
}

impl<T: Spawn> SliderWidget for T {
    fn slider<R: Resource + std::fmt::Debug + From<f32> + Into<f32> + Copy + Clone>(
        &mut self,
        label: &str,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Slider"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::axes(Px(10.0), Px(2.0)),
                    min_width: Px(120.0),
                    height: Auto,
                    ..default()
                },
                background_color: BackgroundColor(SLIDER_BACKGROUND),
                border_radius: BorderRadius::all(Px(4.0)),
                ..default()
            },
        ));

        entity.with_children(|slider| {
            slider
                .spawn((
                    Name::new("Label Container"),
                    NodeBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|label_container| {
                    label_container.spawn((
                        Name::new("Label"),
                        TextBundle::from_section(
                            label,
                            TextStyle {
                                font_size: 16.0,
                                color: TEXT,
                                ..default()
                            },
                        ),
                    ));

                    label_container
                        .spawn((
                            Name::new("Value"),
                            TextBundle::from_section(
                                "-",
                                TextStyle {
                                    font_size: 16.0,
                                    color: TEXT,
                                    ..default()
                                },
                            )
                            .with_text_justify(JustifyText::Right)
                            .with_style(Style {
                                width: Px(40.0),
                                overflow: Overflow {
                                    x: OverflowAxis::Clip,
                                    y: OverflowAxis::Visible,
                                },
                                ..default()
                            }),
                            WatchResource::<R>::new(),
                        ))
                        .observe(update_text::<R>.map(utils::warn));
                });

            slider
                .spawn((
                    Name::new("SliderSlot"),
                    ButtonBundle {
                        style: Style {
                            width: Percent(100.0),
                            height: Px(4.0),
                            margin: UiRect::px(0.0, 0.0, KNOB_PADDING, KNOB_PADDING),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        background_color: BackgroundColor(SLIDER_SLOT),
                        border_radius: BorderRadius::all(Px(4.0)),
                        ..default()
                    },
                    SliderSlot,
                    RelativeCursorPosition::default(),
                ))
                // Observers
                .observe(on_drag::<R>.map(utils::warn))
                // Children
                .with_children(|slot| {
                    slot.spawn((
                        Name::new("Slider Knob"),
                        NodeBundle {
                            style: Style {
                                height: Px(KNOB_HEIGHT),
                                width: Px(KNOB_WIDTH),
                                border: UiRect::all(Px(0.5)),
                                ..default()
                            },
                            border_radius: BorderRadius::all(Percent(100.0)),
                            background_color: BackgroundColor(SLIDER_KNOB),
                            border_color: BorderColor(SLIDER_KNOB_OUTLINE),
                            ..default()
                        },
                        SliderKnob,
                        WatchResource::<R>::new(),
                    ))
                    .observe(update_knob_position::<OnUiNodeSizeChange, R>.map(utils::warn))
                    .observe(update_knob_position::<OnResourceUpdated<R>, R>.map(utils::warn));
                });
        });

        entity
    }
}

#[derive(Component)]
pub struct SliderKnob;

#[derive(Component)]
pub struct SliderSlot;

fn update_knob_position<
    T: Event + std::fmt::Debug,
    R: Resource + std::fmt::Debug + Into<f32> + Copy + Clone,
>(
    trigger: Trigger<T>,
    resource: Res<R>,
    mut knob_q: Query<(&mut Style, &Parent), With<SliderKnob>>,
    slot_q: Query<&Node, With<SliderSlot>>,
) -> Result<()> {
    let (knob_style, knob_parent) = &mut knob_q.get_mut(trigger.entity())?;
    let slot_node = slot_q.get(knob_parent.get())?;
    let resource_value: f32 = (*resource).into();
    let Vec2 {
        x: slot_width,
        y: _,
    } = slot_node.size();
    let percentage = f32::inverse_lerp(1.0, 200.0, resource_value);

    let cubic_bezier = CubicSegment::new_bezier((0.0, 0.5), (0.5, 1.0));
    let percentage = cubic_bezier.ease(percentage);
    let knob_x: f32 = (0.0).lerp(slot_width, percentage) - (KNOB_WIDTH * 0.5) + KNOB_PADDING;
    knob_style.left = Px(knob_x);

    Ok(())
}

fn update_text<R: Resource + Into<f32> + Clone + Copy>(
    trigger: Trigger<OnResourceUpdated<R>>,
    resource: Res<R>,
    mut text_q: Query<&mut Text>,
) -> Result<()> {
    let mut text = text_q.get_mut(trigger.entity())?;
    text.sections[0].value = format!("{0:.2}", (*resource).into());
    Ok(())
}

fn on_drag<R: Resource + std::fmt::Debug + From<f32> + Copy + Clone>(
    trigger: Trigger<OnDrag>,
    mut resource: ResMut<R>,
    slot_q: Query<&RelativeCursorPosition, With<SliderSlot>>,
) -> Result<()> {
    let cursor_pos = slot_q.get(trigger.entity())?;
    if let Some(Vec2 { x, y }) = cursor_pos.normalized {
        let cubic_bezier = CubicSegment::new_bezier((0.5, 0.0), (1.0, 0.5));
        let eased_percentage = cubic_bezier.ease(x);

        *resource = lerp(1.0..=200.0, eased_percentage).into();
    }

    Ok(())
}

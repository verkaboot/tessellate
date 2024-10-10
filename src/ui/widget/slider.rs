use bevy::color::palettes::css::WHITE;
use bevy::utils;
use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};
use bevy_inspector_egui::egui::lerp;

use crate::canvas::mouse::MouseData;
use crate::error::{Error, Result};
use crate::ui::interaction::{OnDrag, OnPress, OnResourceUpdated, WatchResource};
use crate::ui::theme::*;
use crate::ui::widget::Spawn;

pub const PHI: f32 = 1.618033988749894848204586834365638118_f32; // 1.61803401f32
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
                ))
                .with_children(|slot| {
                    slot.spawn((
                        Name::new("Slider Left Bound"),
                        NodeBundle {
                            style: Style {
                                width: Px(0.),
                                height: Px(0.),
                                ..default()
                            },
                            background_color: BackgroundColor(WHITE.into()),
                            ..default()
                        },
                        SliderLeftBound,
                    ));

                    slot.spawn((
                        Name::new("Slider Knob"),
                        ButtonBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
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
                        WatchResource {
                            resource: std::marker::PhantomData::<R>,
                        },
                    ))
                    .observe(update_knob_position::<R>.map(utils::warn))
                    .observe(on_press.map(utils::warn))
                    .observe(on_drag::<R>.map(utils::warn));

                    slot.spawn((
                        Name::new("Slider Right Bound"),
                        NodeBundle {
                            style: Style {
                                width: Px(0.),
                                height: Px(0.),
                                ..default()
                            },
                            background_color: BackgroundColor(WHITE.into()),
                            ..default()
                        },
                        SliderRightBound,
                    ));
                });
        });

        entity
    }
}

#[derive(Component)]
pub struct SliderKnob;

#[derive(Component)]
pub struct SliderSlot;

#[derive(Component)]
pub struct SliderLeftBound;

#[derive(Component)]
pub struct SliderRightBound;

#[derive(Component)]
pub struct MouseOffset(f32);

fn on_press(
    trigger: Trigger<OnPress>,
    mut commands: Commands,
    mouse_data: Res<MouseData>,
    knob_q: Query<&GlobalTransform, With<SliderKnob>>,
) -> Result<()> {
    let knob_x = knob_q
        .get(trigger.entity())
        .map(|global_transform| global_transform.translation().x)?;
    let mouse_x = mouse_data.screen_pos[0].x;
    commands
        .entity(trigger.entity())
        .insert(MouseOffset(knob_x - mouse_x));

    Ok(())
}

fn update_knob_position<R: Resource + std::fmt::Debug + Into<f32> + Copy + Clone>(
    trigger: Trigger<OnResourceUpdated<R>>,
    resource: Res<R>,
    mut knob_q: Query<(&mut Style, &Parent), With<SliderKnob>>,
    slot_q: Query<&Children, With<SliderSlot>>,
    left_bound_q: Query<&GlobalTransform, With<SliderLeftBound>>,
    right_bound_q: Query<&GlobalTransform, With<SliderRightBound>>,
) -> Result<()> {
    let (mut knob_style, parent) = knob_q.get_mut(trigger.entity())?;
    let resource_value: f32 = (*resource).into();
    let slot_children = slot_q.get(parent.get()).unwrap();

    let (left_bound, right_bound) = get_slider_bounds(slot_children, left_bound_q, right_bound_q)?;

    let percentage = f32::inverse_lerp(1.0, 200.0, resource_value);
    let cubic_bezier = CubicSegment::new_bezier((0.0, 0.5), (0.5, 1.0));
    let percentage = cubic_bezier.ease(percentage);
    let knob_x: f32 =
        (0.0).lerp(right_bound - left_bound, percentage) - (KNOB_WIDTH * 0.5) + KNOB_PADDING;
    knob_style.left = Px(knob_x);

    Ok(())
}

fn on_drag<R: Resource + std::fmt::Debug + From<f32> + Copy + Clone>(
    trigger: Trigger<OnDrag>,
    mut resource: ResMut<R>,
    mouse_data: Res<MouseData>,
    knob_q: Query<(&Parent, &MouseOffset), With<SliderKnob>>,
    slot_q: Query<&Children, With<SliderSlot>>,
    left_bound_q: Query<&GlobalTransform, With<SliderLeftBound>>,
    right_bound_q: Query<&GlobalTransform, With<SliderRightBound>>,
) -> Result<()> {
    let (parent, mouse_offset) = knob_q.get(trigger.entity()).unwrap();
    let slot_children = slot_q.get(parent.get()).unwrap();

    let (left_bound, right_bound) = get_slider_bounds(slot_children, left_bound_q, right_bound_q)?;

    let mouse_x = mouse_data.screen_pos[0].x;
    let x = mouse_x + mouse_offset.0;

    let percentage = f32::inverse_lerp(left_bound, right_bound, x).clamp(0.0, 1.0);

    let cubic_bezier = CubicSegment::new_bezier((0.5, 0.0), (1.0, 0.5));
    let eased_percentage = cubic_bezier.ease(percentage);

    *resource = lerp(1.0..=200.0, eased_percentage).into();

    Ok(())
}

fn get_slider_bounds(
    slot_children: &Children,
    left_bound_q: Query<&GlobalTransform, With<SliderLeftBound>>,
    right_bound_q: Query<&GlobalTransform, With<SliderRightBound>>,
) -> Result<(f32, f32)> {
    let (left, right) = slot_children.iter().fold(
        (
            Err(Error::Custom("No left bound found in Slider".into())),
            Err(Error::Custom("No right bound found in Slider".into())),
        ),
        |mut acc, child| {
            if let Ok(left) = left_bound_q.get(*child) {
                acc.0 = Ok(left.translation().x + KNOB_PADDING);
            }
            if let Ok(right) = right_bound_q.get(*child) {
                acc.1 = Ok(right.translation().x - KNOB_PADDING);
            }
            acc
        },
    );
    Ok((left?, right?))
}

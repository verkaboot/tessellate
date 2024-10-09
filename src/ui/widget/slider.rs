use bevy::color::palettes::css::WHITE;
use bevy::utils;
use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};
use bevy_inspector_egui::egui::lerp;

use crate::canvas::mouse::MouseData;
use crate::error::{Error, Result};
use crate::ui::interaction::{OnDrag, OnResourceUpdated, WatchResource};
use crate::ui::theme::*;
use crate::ui::widget::Spawn;

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
                                width: Px(5.),
                                height: Px(5.),
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
                        SliderKnob,
                        WatchResource {
                            resource: std::marker::PhantomData::<R>,
                        },
                    ))
                    .observe(update_knob_position::<R>.map(utils::warn))
                    .observe(on_drag::<R>.map(utils::warn));

                    slot.spawn((
                        Name::new("Slider Right Bound"),
                        NodeBundle {
                            style: Style {
                                width: Px(5.),
                                height: Px(5.),
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

fn update_knob_position<R: Resource + std::fmt::Debug + Into<f32> + Copy + Clone>(
    trigger: Trigger<OnResourceUpdated<R>>,
    resource: Res<R>,
    slot_q: Query<(&GlobalTransform, &Children), With<SliderSlot>>,
    mut knob_q: Query<(&mut Style, &Parent), With<SliderKnob>>,
    left_bound_q: Query<&GlobalTransform, With<SliderLeftBound>>,
    right_bound_q: Query<&GlobalTransform, With<SliderRightBound>>,
) -> Result<()> {
    let (mut knob_style, knob_parent) = knob_q.get_mut(trigger.entity())?;
    let x: f32 = (*resource).into();

    let percentage = get_percentage(1.0, 200.0, x) * 100.0;

    knob_style.left = Percent(percentage);

    Ok(())
}

fn on_drag<R: Resource + std::fmt::Debug + From<f32> + Copy + Clone>(
    trigger: Trigger<OnDrag>,
    mut resource: ResMut<R>,
    mouse_data: Res<MouseData>,
    knob_q: Query<&Parent, With<SliderKnob>>,
    slot_q: Query<(&GlobalTransform, &Children), With<SliderSlot>>,
    left_bound_q: Query<&GlobalTransform, With<SliderLeftBound>>,
    right_bound_q: Query<&GlobalTransform, With<SliderRightBound>>,
) -> Result<()> {
    let parent = knob_q.get(trigger.entity()).unwrap();
    let (slot_transform, slot_children) = slot_q.get(parent.get()).unwrap();
    let left_bound: f32 = slot_children
        .iter()
        .find_map(|child| left_bound_q.get(*child).ok())
        .map(|gt| gt.translation().x)
        .ok_or(Error::Custom("Slider missing a left bound.".into()))?;
    let right_bound: f32 = slot_children
        .iter()
        .find_map(|child| right_bound_q.get(*child).ok())
        .map(|gt| gt.translation().x)
        .ok_or(Error::Custom("Slider missing a right bound.".into()))?;

    let percentage = get_percentage(left_bound, right_bound, mouse_data.screen_pos[0].x);
    println!("{:?}", percentage);

    *resource = lerp(1.0..=200.0, percentage).into();

    Ok(())
}

// Scale the value in the range of the slider
// Value has a min and max
// Slider has a size in Px
// Get percentage of position on the slider and map
// to percentage of range for the value

fn get_percentage(min: f32, max: f32, value: f32) -> f32 {
    (value - min) / (max - min)
}

use std::marker::PhantomData;

use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::ui::RelativeCursorPosition;
use bevy::utils;
use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use crate::interaction::{OnDrag, OnResourceUpdated, OnUiNodeSizeChange, WatchResource};
use crate::theme::*;
use crate::widget::Spawn;
use error::Result;

pub const PHI: f32 = 1.618;
pub const KNOB_HEIGHT: f32 = 14.0;
pub const KNOB_WIDTH: f32 = KNOB_HEIGHT * PHI;
pub const KNOB_PADDING: f32 = KNOB_WIDTH * (2.0 - PHI);

pub struct Slider<R: Resource> {
    phantom_data: PhantomData<R>,
}

impl<R: Resource> Component for Slider<R> {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, component_id| {});
    }
}

pub trait SliderValue {
    fn from_f32(input: f32) -> Self;
    fn to_f32(&self) -> f32;
}

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
            Slider {
                phantom_data: PhantomData::<R>,
            },
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::axes(Px(10.0), Px(2.0)),
                    min_width: Px(120.0),
                    height: Auto,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
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
                    Name::new("Slider Slot"),
                    NodeBundle {
                        style: Style {
                            width: Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            padding: UiRect::vertical(Px(KNOB_PADDING)),
                            ..default()
                        },
                        ..default()
                    },
                    SliderSlot,
                ))
                .with_children(|slot| {
                    slot.spawn((
                        Name::new("Slot Graphic"),
                        NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                width: Percent(100.0),
                                height: Px(KNOB_HEIGHT),
                                border: UiRect::all(Px(1.0)),
                                ..default()
                            },
                            border_color: BorderColor(SLIDER_BACKGROUND),
                            background_color: BackgroundColor(SLIDER_SLOT),
                            border_radius: BorderRadius::all(Percent(100.0)),
                            ..default()
                        },
                    ));

                    let graphic_fill = slot
                        .spawn((
                            Name::new("Slot Graphic Fill"),
                            NodeBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    width: Percent(50.0),
                                    height: Px(KNOB_HEIGHT),
                                    border: UiRect::all(Px(1.0)),
                                    ..default()
                                },
                                border_color: BorderColor(SLIDER_BACKGROUND),
                                background_color: BackgroundColor(SLIDER_SLOT_FILL),
                                border_radius: BorderRadius::percent(100.0, 0.0, 0.0, 100.0),
                                ..default()
                            },
                            GraphicFill,
                        ))
                        .id();

                    slot.spawn((
                        Name::new("KnobContainer"),
                        NodeBundle {
                            style: Style {
                                width: Percent(100.0),
                                // Make container smaller than the graphic to fit knob
                                margin: UiRect::horizontal(Px(KNOB_WIDTH * 0.5)),
                                ..default()
                            },
                            ..default()
                        },
                        KnobContainer,
                        RelativeCursorPosition::default(),
                    ))
                    .with_children(|knob_container| {
                        knob_container
                            .spawn((
                                Name::new("Slider Knob"),
                                ButtonBundle {
                                    style: Style {
                                        height: Px(KNOB_HEIGHT),
                                        width: Px(KNOB_WIDTH),
                                        border: UiRect::all(Px(1.0)),
                                        // Center the knob
                                        margin: UiRect::left(Px(KNOB_WIDTH * -0.5)),
                                        ..default()
                                    },
                                    border_radius: BorderRadius::all(Percent(100.0)),
                                    background_color: BackgroundColor(SLIDER_KNOB),
                                    border_color: BorderColor(SLIDER_KNOB_OUTLINE),
                                    ..default()
                                },
                                SliderKnob,
                                FillEntity(graphic_fill),
                                WatchResource::<R>::new(),
                            ))
                            .observe(on_drag::<R>.map(utils::warn))
                            .observe(update_knob_position::<OnUiNodeSizeChange, R>.map(utils::warn))
                            .observe(
                                update_knob_position::<OnResourceUpdated<R>, R>.map(utils::warn),
                            );
                    });
                });
        });

        entity
    }
}

#[derive(Component)]
pub struct KnobContainer;

#[derive(Component)]
pub struct SliderKnob;

#[derive(Component)]
pub struct SliderSlot;

#[derive(Component)]
pub struct GraphicFill;

#[derive(Component)]
pub struct FillEntity(Entity);

fn on_drag<R: Resource + std::fmt::Debug + From<f32> + Copy + Clone>(
    trigger: Trigger<OnDrag>,
    mut resource: ResMut<R>,
    container_q: Query<&RelativeCursorPosition, With<KnobContainer>>,
    knob_q: Query<&Parent, With<SliderKnob>>,
) -> Result<()> {
    let knob_parent = knob_q.get(trigger.entity())?;
    let cursor_pos = container_q.get(knob_parent.get())?;
    if let Some(Vec2 { x, y: _ }) = cursor_pos.normalized {
        let cubic_bezier = CubicSegment::new_bezier((0.5, 0.0), (1.0, 0.5));
        let eased_percentage = cubic_bezier.ease(x);

        *resource = 1.0.lerp(200.0, eased_percentage).into();
    }

    Ok(())
}

fn update_knob_position<
    T: Event + std::fmt::Debug,
    R: Resource + std::fmt::Debug + Into<f32> + Copy + Clone,
>(
    trigger: Trigger<T>,
    resource: Res<R>,
    mut knob_q: Query<(&mut Style, &FillEntity), With<SliderKnob>>,
    mut fill_q: Query<&mut Style, (With<GraphicFill>, Without<SliderKnob>)>,
) -> Result<()> {
    let (knob_style, fill_entity) = &mut knob_q.get_mut(trigger.entity())?;
    let mut fill_style = fill_q.get_mut(fill_entity.0)?;
    let resource_value: f32 = (*resource).into();
    let percentage = f32::inverse_lerp(1.0, 200.0, resource_value);

    let cubic_bezier = CubicSegment::new_bezier((0.0, 0.5), (0.5, 1.0));
    let percentage = cubic_bezier.ease(percentage);
    knob_style.left = Percent(percentage * 100.0);
    fill_style.width = Percent(percentage * 100.0);

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

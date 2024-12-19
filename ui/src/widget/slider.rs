use bevy::ui::RelativeCursorPosition;
use bevy::utils;
use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};
use input::trigger::{Drag, OnPress, OnResourceUpdated, OnUiNodeSizeChange, WatchResource};

use crate::theme::*;
use crate::widget::Spawn;
use error::Result;

pub const PHI: f32 = 1.618;
pub const KNOB_HEIGHT: f32 = 14.0;
pub const KNOB_WIDTH: f32 = KNOB_HEIGHT * PHI;
pub const KNOB_PADDING: f32 = KNOB_WIDTH * (2.0 - PHI);

pub trait SliderValue: Resource + Copy + Clone + std::fmt::Debug {
    fn from_f32(input: f32) -> Self;
    fn to_f32(&self) -> f32;
}

pub trait SliderWidget {
    fn slider<V: SliderValue>(
        &mut self,
        label: &str,
        min_value: f32,
        max_value: f32,
    ) -> EntityCommands;
}

impl<T: Spawn> SliderWidget for T {
    fn slider<V: SliderValue>(
        &mut self,
        label: &str,
        min_value: f32,
        max_value: f32,
    ) -> EntityCommands {
        let mut entity = self.ui_spawn((
            Name::new("Slider"),
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::axes(Px(10.0), Px(2.0)),
                width: Percent(100.0),
                height: Auto,
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(SLIDER_BACKGROUND),
            BorderRadius::all(Px(4.0)),
        ));

        entity.with_children(|slider| {
            slider
                .ui_spawn((
                    Name::new("Label Container"),
                    Node {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                ))
                .with_children(|label_container| {
                    label_container.ui_spawn((
                        Name::new("Label"),
                        Text(label.into()),
                        TextFont {
                            font_size: FONT_SIZE,
                            ..default()
                        },
                    ));

                    label_container
                        .ui_spawn((
                            Name::new("Value"),
                            Text("-".into()),
                            TextFont {
                                font_size: FONT_SIZE,
                                ..default()
                            },
                            WatchResource::<V>::new(),
                        ))
                        .observe(update_text::<V>);
                });

            slider
                .ui_spawn((
                    Name::new("Slider Slot"),
                    Node {
                        width: Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::vertical(Px(KNOB_PADDING)),
                        ..default()
                    },
                    SliderSlot,
                ))
                .with_children(|slot| {
                    slot.ui_spawn((
                        Name::new("Slot Graphic"),
                        Node {
                            position_type: PositionType::Absolute,
                            width: Percent(100.0),
                            height: Px(KNOB_HEIGHT),
                            border: UiRect::all(Px(1.0)),
                            ..default()
                        },
                        BorderColor(SLIDER_BACKGROUND),
                        BackgroundColor(SLIDER_SLOT),
                        BorderRadius::all(Percent(100.0)),
                    ));

                    let graphic_fill = slot
                        .ui_spawn((
                            Name::new("Slot Graphic Fill"),
                            Node {
                                position_type: PositionType::Absolute,
                                width: Percent(50.0),
                                height: Px(KNOB_HEIGHT),
                                border: UiRect::all(Px(1.0)),
                                ..default()
                            },
                            BorderColor(SLIDER_BACKGROUND),
                            BackgroundColor(SLIDER_SLOT_FILL),
                            BorderRadius::percent(100.0, 0.0, 0.0, 100.0),
                            GraphicFill,
                        ))
                        .id();

                    slot.ui_spawn((
                        Name::new("KnobContainer"),
                        KnobContainer,
                        RelativeCursorPosition::default(),
                        Button,
                        Node {
                            width: Percent(100.0),
                            // Make container smaller than the graphic to fit knob
                            margin: UiRect::horizontal(Px(KNOB_WIDTH * 0.5)),
                            ..default()
                        },
                    ))
                    .with_children(|knob_container| {
                        knob_container
                            .ui_spawn((
                                Name::new("Slider Knob"),
                                Button,
                                Node {
                                    height: Px(KNOB_HEIGHT),
                                    width: Px(KNOB_WIDTH),
                                    border: UiRect::all(Px(1.0)),
                                    // Center the knob
                                    margin: UiRect::left(Px(KNOB_WIDTH * -0.5)),
                                    ..default()
                                },
                                BorderRadius::all(Percent(100.0)),
                                BackgroundColor(SLIDER_KNOB),
                                BorderColor(SLIDER_KNOB_OUTLINE),
                                SliderKnob,
                                SliderValueRange {
                                    min: min_value,
                                    max: max_value,
                                },
                                FillEntity(graphic_fill),
                                RelativeCursorPosition::default(),
                                WatchResource::<V>::new(),
                            ))
                            .observe(on_drag::<V>.map(utils::warn))
                            .observe(update_knob_position::<OnUiNodeSizeChange, V>.map(utils::warn))
                            .observe(
                                update_knob_position::<OnResourceUpdated<V>, V>.map(utils::warn),
                            );
                    })
                    .observe(on_press::<V>.map(utils::warn));
                });
        });

        entity
    }
}

#[derive(Component)]
pub struct KnobContainer;

#[derive(Component)]
pub struct SliderValueRange {
    min: f32,
    max: f32,
}

#[derive(Component)]
pub struct SliderKnob;

#[derive(Component)]
pub struct SliderSlot;

#[derive(Component)]
pub struct GraphicFill;

#[derive(Component)]
pub struct FillEntity(Entity);

// When we press the knob container
fn on_press<V: SliderValue>(
    trigger: Trigger<OnPress>,
    mut resource: ResMut<V>,
    cursor_q: Query<&RelativeCursorPosition>,
    knob_q: Query<(&SliderValueRange,), With<SliderKnob>>,
    children_q: Query<&Children>,
) -> Result<()> {
    let cursor_pos = cursor_q.get(trigger.entity())?;
    let children = children_q.get(trigger.entity())?;
    let knob_entity = children.iter().find(|&&child| knob_q.contains(child));
    if let Some(knob_entity) = knob_entity {
        let (range,) = knob_q.get(*knob_entity)?;
        if let Some(Vec2 { x, y: _ }) = cursor_pos.normalized {
            let cubic_bezier = CubicSegment::new_bezier((0.5, 0.0), (1.0, 0.5));
            let eased_percentage = cubic_bezier.ease(x);

            *resource = V::from_f32(range.min.lerp(range.max, eased_percentage));
        }
    }

    Ok(())
}

// When we drag the knob
fn on_drag<V: SliderValue>(
    trigger: Trigger<Drag>,
    mut resource: ResMut<V>,
    container_q: Query<&RelativeCursorPosition, With<KnobContainer>>,
    knob_q: Query<(&Parent, &SliderValueRange), With<SliderKnob>>,
) -> Result<()> {
    let (knob_parent, range) = knob_q.get(trigger.entity())?;
    let cursor_pos = container_q.get(knob_parent.get())?;
    if let Some(Vec2 { x, y: _ }) = cursor_pos.normalized {
        let cubic_bezier = CubicSegment::new_bezier((0.5, 0.0), (1.0, 0.5));
        let eased_percentage = cubic_bezier.ease(x);

        *resource = V::from_f32(range.min.lerp(range.max, eased_percentage));
    }

    Ok(())
}

fn update_knob_position<T: Event + std::fmt::Debug, V: SliderValue>(
    trigger: Trigger<T>,
    slider_value: Res<V>,
    mut knob_q: Query<(&mut Node, &FillEntity, &SliderValueRange), With<SliderKnob>>,
    mut fill_q: Query<&mut Node, (With<GraphicFill>, Without<SliderKnob>)>,
) -> Result<()> {
    let (knob_style, fill_entity, range) = &mut knob_q.get_mut(trigger.entity())?;
    let mut fill_style = fill_q.get_mut(fill_entity.0)?;
    let resource_value: f32 = slider_value.to_f32();
    let percentage = f32::inverse_lerp(range.min, range.max, resource_value);

    let cubic_bezier = CubicSegment::new_bezier((0.0, 0.5), (0.5, 1.0));
    let percentage = cubic_bezier.ease(percentage);
    knob_style.left = Percent(percentage * 100.0);
    fill_style.width = Percent(percentage * 100.0);

    Ok(())
}

fn update_text<V: SliderValue>(
    trigger: Trigger<OnResourceUpdated<V>>,
    mut writer: TextUiWriter,
    resource: Res<V>,
) {
    *writer.text(trigger.entity(), 0) = format!("{0:.2}", (*resource).to_f32());
}

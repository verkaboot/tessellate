use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use crate::canvas::mouse::MouseData;
use crate::ui::interaction::{OnDrag, OnPress, OnRelease, OnResourceUpdated, WatchResource};
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
                            padding: UiRect::right(Px(12.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Start,
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
                        SliderKnob,
                        WatchResource {
                            resource: std::marker::PhantomData::<R>,
                        },
                    ))
                    .observe(update_knob_position::<R>)
                    .observe(
                        |trigger: Trigger<OnDrag>,
                         mut resource: ResMut<R>,
                         mouse_data: Res<MouseData>,
                         knob_q: Query<&Parent, With<SliderKnob>>,
                         slot_q: Query<&GlobalTransform, With<SliderSlot>>| {
                            let parent = knob_q.get(trigger.entity()).unwrap();
                            let slot_transform = slot_q.get(parent.get()).unwrap();
                            *resource = (mouse_data.screen_pos[0].x - slot_transform.translation().x).into();
                        },
                    );
                });
        });

        entity
    }
}

#[derive(Component)]
pub struct SliderKnob;

#[derive(Component)]
pub struct SliderSlot;

fn update_knob_position<R: Resource + std::fmt::Debug + Into<f32> + Copy + Clone>(
    trigger: Trigger<OnResourceUpdated<R>>,
    resource: Res<R>,
    mut knob_q: Query<&mut Style, With<SliderKnob>>,
) {
    println!("OnResourceUpdate: {:?}", resource);
    let mut style = knob_q.get_mut(trigger.entity()).unwrap();
    style.left = Px((*resource).into());
}

// TODO: In order to get the bounds of the slot, add two node components to the beginning and end of the row. Make them 1x1 pixels, then you can get their GlobalTransforms

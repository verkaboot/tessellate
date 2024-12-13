use bevy::{prelude::*, ui::RelativeCursorPosition};
use std::marker::PhantomData;

use crate::interaction::PreviousInteraction;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            trigger_on_press,
            trigger_on_release,
            trigger_on_drag,
            trigger_node_updated,
        ),
    );
}

// TODO: Try implementing composable triggers
// by using a trait to give each trigger its
// update function (like trigger_on_press for Press)
// which might allow And to check two functions.
// The update functions probably need two parts:
// a system that returns a boolean to determine
// if it should run, and then a system that runs
// the trigger and any side effects.
pub trait CustomTrigger {}

#[derive(Event)]
pub struct WithKey<E: Event> {
    key: KeyCode,
    e: E,
}

fn trigger_on_press(
    interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    commands.trigger_targets(OnPress, entity);
}

#[derive(Event)]
pub struct OnPress;

fn trigger_on_press(
    interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (entity, interaction) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            commands.trigger_targets(OnPress, entity);
            commands
                .entity(entity)
                .insert(PreviousInteraction(*interaction));
        }
    }
}

#[derive(Event)]
pub struct Drag;

fn trigger_on_drag(
    interaction_query: Query<(Entity, &Interaction), Changed<RelativeCursorPosition>>,
    mut commands: Commands,
) {
    for (entity, interaction) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            commands.trigger_targets(Drag, entity);
        }
    }
}

#[derive(Event)]
pub struct OnRelease;

fn trigger_on_release(
    interaction_query: Query<
        (Entity, &Interaction, Option<&PreviousInteraction>),
        Changed<Interaction>,
    >,
    mut commands: Commands,
) {
    for (entity, interaction, previous_interaction) in &interaction_query {
        if matches!(
            previous_interaction,
            Some(PreviousInteraction(Interaction::Pressed))
        ) && matches!(interaction, Interaction::Hovered | Interaction::None)
        {
            commands.trigger_targets(OnRelease, entity);
        }
    }
}

#[derive(Event, Debug)]
pub struct OnResourceUpdated<R: Resource> {
    resource: PhantomData<R>,
}

#[derive(Component)]
pub struct WatchResource<R: Resource> {
    pub resource: PhantomData<R>,
}

impl<R: Resource> WatchResource<R> {
    pub fn new() -> Self {
        Self {
            resource: PhantomData::<R>,
        }
    }
}

pub fn trigger_on_resource_updated<R: Resource>(
    watcher_q: Query<Entity, With<WatchResource<R>>>,
    resource: Res<R>,
    mut commands: Commands,
) {
    if resource.is_changed() {
        for entity in &watcher_q {
            commands.trigger_targets(
                OnResourceUpdated {
                    resource: std::marker::PhantomData::<R>,
                },
                entity,
            );
        }
    }
}

pub fn trigger_watch_resource_init<R: Resource>(
    watcher_q: Query<Entity, Added<WatchResource<R>>>,
    mut commands: Commands,
) {
    for entity in &watcher_q {
        commands.trigger_targets(
            OnResourceUpdated {
                resource: std::marker::PhantomData::<R>,
            },
            entity,
        );
    }
}

#[derive(Event, Debug)]
pub struct OnUiNodeSizeChange;

fn trigger_node_updated(watcher_q: Query<Entity, Changed<Node>>, mut commands: Commands) {
    for entity in &watcher_q {
        commands.trigger_targets(OnUiNodeSizeChange, entity);
    }
}

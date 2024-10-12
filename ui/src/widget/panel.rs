use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use crate::theme::*;
use crate::widget::Spawn;

pub trait PanelWidget {
    fn panel(&mut self, direction: PanelDirection) -> EntityCommands;
}

pub enum PanelDirection {
    Wide,
    Tall,
}

impl<T: Spawn> PanelWidget for T {
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
}

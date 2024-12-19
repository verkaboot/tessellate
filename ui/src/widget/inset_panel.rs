use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use crate::theme::*;
use crate::widget::Spawn;

pub trait InsetPanelWidget {
    fn inset_panel(&mut self) -> EntityCommands;
}

impl<T: Spawn> InsetPanelWidget for T {
    fn inset_panel(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("Panel"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Auto,
                    flex_direction: FlexDirection::Column,
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
                background_color: BackgroundColor(INSET_PANEL_BACKGROUND),
                ..default()
            },
            Outline::new(Px(1.0), Val::ZERO, PANEL_OUTLINE),
        ))
    }
}

use bevy::prelude::*;
use sickle_ui::{prelude::*, SickleUiPlugin};

use crate::canvas::brush::BrushSize;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(SickleUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_brush_size);
}

#[derive(Component)]
pub struct BrushSizeSlider;

fn setup(mut commands: Commands, brush_size: Res<BrushSize>) {
    commands.ui_builder(UiRoot).row(|row| {
        row.slider(SliderConfig::horizontal(
            Some("Brush Size".to_owned()),
            1.0,
            300.,
            **brush_size,
            true,
        ))
        .insert(BrushSizeSlider);
    });
}

fn update_brush_size(
    slider_q: Query<&Slider, (With<BrushSizeSlider>, Changed<Slider>)>,
    mut brush_size: ResMut<BrushSize>,
) {
    if let Ok(slider) = slider_q.get_single() {
        **brush_size = slider.value();
    }
}

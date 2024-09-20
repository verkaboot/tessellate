use bevy::prelude::*;
use sickle_ui::{prelude::*, SickleUiPlugin};

use crate::canvas::brush::{BrushColor, BrushSize};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(SickleUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_brush_size, update_brush_hue));
}

#[derive(Component)]
pub struct BrushSizeSlider;

#[derive(Component)]
pub struct BrushHueSlider;

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

        row.slider(SliderConfig::horizontal(
            Some("Hue".to_owned()),
            1.0,
            255.,
            **brush_size,
            true,
        ))
        .insert(BrushHueSlider);
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

fn update_brush_hue(
    slider_q: Query<&Slider, (With<BrushHueSlider>, Changed<Slider>)>,
    mut brush_color: ResMut<BrushColor>,
) {
    if let Ok(slider) = slider_q.get_single() {
        brush_color.set_hue(slider.value());
    }
}

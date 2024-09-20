use bevy::{prelude::*, utils};
use sickle_ui::{prelude::*, SickleUiPlugin};

use crate::{canvas::brush::BrushSize, error::Result};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(SickleUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_brush_size.map(utils::warn));
}

#[derive(Component)]
pub struct BrushSizeSlider;

fn setup(mut commands: Commands) {
    commands.ui_builder(UiRoot).row(|row| {
        row.slider(SliderConfig::horizontal(
            Some("Brush Size".to_owned()),
            1.0,
            300.,
            5.0,
            true,
        ))
        .insert(BrushSizeSlider);
    });
}

fn update_brush_size(
    slider_q: Query<&Slider, With<BrushSizeSlider>>,
    mut brush_size: ResMut<BrushSize>,
) -> Result<()> {
    let slider = slider_q.get_single()?;
    brush_size.0 = slider.value();

    Ok(())
}

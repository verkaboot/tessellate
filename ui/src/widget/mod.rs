mod button;
mod canvas;
pub mod color_picker;
mod flex;
mod panel;
mod root;
mod slider;

use bevy::{ecs::system::EntityCommands, prelude::*};
use color_picker::ColorPickerPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ColorPickerPlugin,));
}

pub mod prelude {
    pub use super::{
        button::ButtonWidget,
        canvas::CanvasWidget,
        flex::FlexWidget,
        panel::{PanelDirection, PanelWidget},
        root::Containers,
        slider::{SliderValue, SliderWidget},
    };
}

trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

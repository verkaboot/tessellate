mod button;
mod canvas;
mod flex;
mod panel;
mod root;
mod slider;

use bevy::{ecs::system::EntityCommands, prelude::*};

pub mod prelude {
    pub use super::{
        button::ButtonWidget,
        canvas::CanvasWidget,
        flex::FlexWidget,
        panel::{PanelDirection, PanelWidget},
        root::Containers,
        slider::SliderWidget,
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

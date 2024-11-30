mod button;
mod canvas;
pub mod color_picker;
mod flex;
mod inset_panel;
mod list;
mod panel;
mod root;
mod slider;
mod text;

use bevy::{ecs::system::EntityCommands, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        UiMaterialPlugin::<color_picker::HueWheelMaterial>::default(),
        UiMaterialPlugin::<color_picker::HsvBoxMaterial>::default(),
    ));
}

pub mod prelude {
    pub use super::{
        button::ButtonWidget,
        canvas::CanvasWidget,
        flex::FlexWidget,
        inset_panel::InsetPanelWidget,
        list::{ListWidget, SelectList},
        panel::{PanelDirection, PanelWidget},
        root::{set_root, watch_state, Containers, CurrentState, RootState},
        slider::{SliderValue, SliderWidget},
        text::{TextValue, TextWidget},
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

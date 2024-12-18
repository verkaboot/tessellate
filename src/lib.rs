mod background;
mod camera;
mod controls;
mod dev;
pub mod event;
mod grid;
mod paint;
mod screen;
mod terrain;

use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Workspace Plugins
        app.add_plugins((
            background::plugin,
            camera::plugin,
            canvas::plugin,
            event::plugin,
            input::plugin,
            screen::plugin,
            terrain::plugin,
            ui::plugin,
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(dev::plugin);

        app.add_systems(Startup, screen::setup);
    }
}

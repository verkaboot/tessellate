mod background;
mod camera;
mod controls;
mod dev;
mod grid;
mod screen;
mod terrain;

use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Workspace Plugins
        app.add_plugins((
            input::plugin,
            camera::plugin,
            canvas::plugin,
            background::plugin,
            ui::plugin,
            screen::plugin,
            terrain::plugin,
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(dev::plugin);

        app.add_systems(Startup, screen::setup);
    }
}

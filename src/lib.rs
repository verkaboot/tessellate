mod background;
mod camera;
mod dev;
mod grid;
mod input;
mod screen;
mod terrain;

use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Workspace Plugins
        app.add_plugins((
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

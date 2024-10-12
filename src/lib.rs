mod background;
mod camera;
mod screen;

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
        ));

        app.add_systems(Startup, screen::setup);
    }
}

mod art_tile;
mod background;
pub mod button;
mod camera;
mod dev;
mod paint;
mod screen;
mod terrain;

use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Workspace Plugins
        app.add_plugins((
            art_tile::plugin,
            background::plugin,
            button::plugin,
            camera::plugin,
            canvas::plugin,
            event::plugin,
            input::plugin,
            paint::plugin,
            screen::plugin,
            terrain::plugin,
            ui::plugin,
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(dev::plugin);

        app.add_systems(Startup, screen::setup);
    }
}

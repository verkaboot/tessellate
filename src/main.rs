mod camera;
mod canvas;
mod color;
mod draw;

use bevy::{prelude::*, winit::WinitSettings};
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{debug::DebugPickingMode, DefaultPickingPlugins};

fn main() {
    App::new()
        // Bevy Plugin Setup
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Animaboot".into(),
                resolution: (1280., 720.).into(),
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(color::BACKGROUND))
        .insert_resource(WinitSettings::desktop_app())
        // 3rd Party Plugins
        .add_plugins(bevy_framepace::FramepacePlugin)
        .insert_resource(FramepaceSettings {
            limiter: Limiter::from_framerate(144.0),
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .insert_resource(DebugPickingMode::Normal)
        // App Plugins
        .add_plugins((camera::plugin, draw::plugin))
        .run();
}

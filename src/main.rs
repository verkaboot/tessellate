mod background;
mod camera;
mod canvas;
mod color;
mod error;

use bevy::{prelude::*, window::WindowResolution, winit::WinitSettings};
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        // Bevy Plugin Setup
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Animaboot".into(),
                        resolution: WindowResolution::new(1920., 1080.)
                            .with_scale_factor_override(1.0),
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(WinitSettings::desktop_app())
        // 3rd Party Plugins
        .add_plugins(bevy_framepace::FramepacePlugin)
        .insert_resource(FramepaceSettings {
            limiter: Limiter::from_framerate(144.0),
        })
        .add_plugins(WorldInspectorPlugin::new())
        // App Plugins
        .add_plugins((camera::plugin, canvas::plugin, background::plugin))
        .run();
}

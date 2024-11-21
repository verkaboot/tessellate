// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::{asset::load_internal_binary_asset, prelude::*};
use bevy::{window::WindowResolution, winit::WinitSettings};
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use tessellate::AppPlugin;

fn main() -> AppExit {
    let mut app = App::new();

    // Bevy Plugin Setup
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Verkaboot Tessellate".into(),
                    resolution: WindowResolution::new(1920., 1080.).with_scale_factor_override(1.0),
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
    .add_plugins(WorldInspectorPlugin::new());

    // Build default font into the binary
    load_internal_binary_asset!(
        app,
        TextStyle::default().font,
        "../assets/fonts/NotoSans.ttf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    app.add_plugins(AppPlugin).run()
}

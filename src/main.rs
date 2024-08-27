use bevy::{prelude::*, winit::WinitSettings};

use bevy_framepace::{FramepaceSettings, Limiter};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Animaboot".into(),
                resolution: (1280., 720.).into(),
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy_framepace::FramepacePlugin)
        .insert_resource(FramepaceSettings {
            limiter: Limiter::from_framerate(144.0),
        })
        .insert_resource(ClearColor(Color::srgb_u8(30, 40, 50)))
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup_camera)
        .run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

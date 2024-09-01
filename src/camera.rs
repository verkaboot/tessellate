use bevy::prelude::*;

use crate::canvas::SIZE;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera);
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(
                SIZE.0 as f32 / 2.0,
                SIZE.1 as f32 / 2.0,
                0.0,
            )),
            ..default()
        },
        IsDefaultUiCamera,
    ));
}

use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    ui::Val::*,
};

use super::Spawn;

pub trait ColorPickerWidget {
    fn color_picker(
        &mut self,
        hue_wheel_material: ResMut<Assets<HueWheelMaterial>>,
        hsv_box_material: ResMut<Assets<HsvBoxMaterial>>,
    ) -> EntityCommands;
}

impl<T: Spawn> ColorPickerWidget for T {
    fn color_picker(
        &mut self,
        mut hue_wheel_material: ResMut<Assets<HueWheelMaterial>>,
        mut hsv_box_material: ResMut<Assets<HsvBoxMaterial>>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("ColorPicker Parent"),
            NodeBundle {
                style: Style {
                    width: Px(300.0),
                    height: Px(300.0),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ));

        entity.with_children(|parent| {
            parent.spawn((
                Name::new("ColorPicker Hue Wheel"),
                MaterialNodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    material: hue_wheel_material.add(HueWheelMaterial {
                        color: LinearRgba::WHITE.to_f32_array().into(),
                    }),
                    ..default()
                },
            ));
        });

        entity.with_children(|parent| {
            parent.spawn((
                Name::new("ColorPicker HSV Box"),
                MaterialNodeBundle {
                    style: Style {
                        width: Val::Percent(54.0),
                        height: Val::Percent(54.0),
                        ..default()
                    },
                    material: hsv_box_material.add(HsvBoxMaterial {
                        hsva: Hsva::hsv(0.0, 0.5, 0.5).to_f32_array().into(),
                    }),
                    ..default()
                },
            ));
        });

        entity
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct HueWheelMaterial {
    #[uniform(0)]
    color: Vec4,
}

impl UiMaterial for HueWheelMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hue_wheel.wgsl".into()
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct HsvBoxMaterial {
    #[uniform(0)]
    hsva: Vec4,
}

impl UiMaterial for HsvBoxMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hsv_box.wgsl".into()
    }
}

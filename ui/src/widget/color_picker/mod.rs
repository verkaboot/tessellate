mod shader;

use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_graph::{self, RenderGraph, RenderLabel},
        render_resource::*,
        renderer::RenderContext,
        Render, RenderApp, RenderSet,
    },
    ui::Val::*,
};

use super::Spawn;

#[derive(Resource, Clone, ExtractResource)]
pub struct ColorPickerImages {
    hue_wheel: Handle<Image>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct ColorPickerLabel;

struct ColorPickerNode;
impl render_graph::Node for ColorPickerNode {
    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let bind_group = &world.resource::<shader::BindGroups>().bind_group;
        let pipeline = world.resource::<shader::Pipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        let update_pipeline = pipeline_cache
            .get_compute_pipeline(pipeline.update_pipeline)
            .unwrap();
        pass.set_bind_group(0, &bind_group, &[]);
        pass.set_pipeline(update_pipeline);
        pass.dispatch_workgroups(8, 8, 1);

        Ok(())
    }
}

pub struct ColorPickerPlugin;
impl Plugin for ColorPickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<ColorPickerImages>::default());
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            shader::bind_groups.in_set(RenderSet::PrepareBindGroups),
        );

        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(ColorPickerLabel, ColorPickerNode);
        render_graph.add_node_edge(ColorPickerLabel, bevy::render::graph::CameraDriverLabel);
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<shader::Pipeline>();
    }
}

fn setup_textures(mut commands: Commands, mut images: ColorPickerImages) {
    let mut hue_wheel = Image::new_fill(
        Extent3d {
            width: 256,
            height: 256,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );
    hue_wheel.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let sprite_image_handle = images.add(hue_wheel);
    commands.insert_resource(ColorPickerImages { hue_wheel });
}

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

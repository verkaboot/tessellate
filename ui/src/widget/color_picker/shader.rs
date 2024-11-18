use std::borrow::Cow;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets, render_resource::*, renderer::RenderDevice, texture::GpuImage,
    },
};
use binding_types::texture_storage_2d;

use super::ColorPickerImages;

const SHADER_ASSET_PATH: &str = "shaders/hue_wheel.wgsl";

#[derive(Resource)]
pub struct Pipeline {
    pub texture_bind_group_layout: BindGroupLayout,
    pub update_pipeline: CachedComputePipelineId,
}

impl FromWorld for Pipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let texture_bind_group_layout = render_device.create_bind_group_layout(
            "ColorPickerImages",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::WriteOnly),
                    texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::WriteOnly),
                ),
            ),
        );
        let shader = world.load_asset(SHADER_ASSET_PATH);
        let pipeline_cache = world.resource::<PipelineCache>();
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });

        Pipeline {
            texture_bind_group_layout,
            update_pipeline,
        }
    }
}

#[derive(Resource)]
pub struct BindGroups {
    pub bind_group: BindGroup,
}

pub fn bind_groups(
    mut commands: Commands,
    pipeline: Res<Pipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    render_device: Res<RenderDevice>,
    color_picker_images: Res<ColorPickerImages>,
) {
    let layered_texture = gpu_images.get(&color_picker_images.hue_wheel).unwrap();

    let bind_group = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((&layered_texture.texture_view,)),
    );

    commands.insert_resource(BindGroups { bind_group });
}

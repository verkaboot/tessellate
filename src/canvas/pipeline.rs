use super::SHADER_ASSET_PATH;
use bevy::{
    prelude::*,
    render::{
        render_resource::{binding_types::texture_storage_2d, *},
        renderer::RenderDevice,
    },
};
use binding_types::uniform_buffer;
use std::borrow::Cow;

#[derive(Resource)]
pub struct CanvasPipeline {
    pub texture_bind_group_layout: BindGroupLayout,
    pub update_pipeline: CachedComputePipelineId,
}

impl FromWorld for CanvasPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let texture_bind_group_layout = render_device.create_bind_group_layout(
            "CanvasImages",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::ReadOnly),
                    texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::WriteOnly),
                    uniform_buffer::<Vec2>(false),
                ),
            ),
        );
        let shader = world.load_asset(SHADER_ASSET_PATH);
        let pipeline_cache = world.resource::<PipelineCache>();
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });

        CanvasPipeline {
            texture_bind_group_layout,
            update_pipeline,
        }
    }
}

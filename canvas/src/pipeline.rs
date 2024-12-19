use super::{tool::MousePositions, SHADER_ASSET_PATH};
use bevy::{
    prelude::*,
    render::{render_resource::*, renderer::RenderDevice},
};
use binding_types::{
    storage_buffer_read_only, texture_storage_2d, texture_storage_2d_array, uniform_buffer,
};
use std::borrow::Cow;

#[derive(Resource)]
pub struct CanvasPipeline {
    pub texture_bind_group_layout: BindGroupLayout,
    pub init_pipeline: CachedComputePipelineId,
    pub paint_normal_pipeline: CachedComputePipelineId,
    pub paint_erase_pipeline: CachedComputePipelineId,
}

impl FromWorld for CanvasPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let texture_bind_group_layout = render_device.create_bind_group_layout(
            "CanvasImages",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_2d_array(
                        TextureFormat::Rgba8Unorm,
                        StorageTextureAccess::ReadWrite,
                    ),
                    texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::WriteOnly),
                    uniform_buffer::<u32>(false),
                    storage_buffer_read_only::<MousePositions>(false),
                    uniform_buffer::<f32>(false),      // BrushSize
                    uniform_buffer::<f32>(false),      // BrushHardness
                    uniform_buffer::<[f32; 4]>(false), // BrushColor
                    storage_buffer_read_only::<Vec<Vec2>>(false),
                ),
            ),
        );
        let shader = world.load_asset(SHADER_ASSET_PATH);
        let pipeline_cache = world.resource::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
            zero_initialize_workgroup_memory: false,
        });

        let paint_normal_pipeline =
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: None,
                layout: vec![texture_bind_group_layout.clone()],
                push_constant_ranges: Vec::new(),
                shader: shader.clone(),
                shader_defs: vec![],
                entry_point: Cow::from("paint_normal"),
                zero_initialize_workgroup_memory: false,
            });

        let paint_erase_pipeline =
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: None,
                layout: vec![texture_bind_group_layout.clone()],
                push_constant_ranges: Vec::new(),
                shader,
                shader_defs: vec![],
                entry_point: Cow::from("paint_erase"),
                zero_initialize_workgroup_memory: false,
            });

        CanvasPipeline {
            texture_bind_group_layout,
            init_pipeline,
            paint_normal_pipeline,
            paint_erase_pipeline,
        }
    }
}

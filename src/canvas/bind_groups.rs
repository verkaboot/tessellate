use super::{mouse::MouseData, pipeline::CanvasPipeline, sprite::CanvasImages};
use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets, render_resource::*, renderer::RenderDevice, texture::GpuImage,
    },
};

#[derive(Resource)]
pub struct CanvasImageBindGroups {
    pub bind_group: BindGroup,
}

pub fn prepare(
    mut commands: Commands,
    pipeline: Res<CanvasPipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    canvas_images: Res<CanvasImages>,
    mouse_position: Res<MouseData>,
    render_device: Res<RenderDevice>,
) {
    let view = gpu_images.get(&canvas_images.texture).unwrap();

    let mouse_position_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&[mouse_position.world_pos]),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });
    let mouse_position_binding = BufferBinding {
        buffer: &mouse_position_buffer,
        offset: 0,
        size: None,
    };

    let bind_group_0 = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((&view.texture_view, mouse_position_binding.clone())),
    );
    commands.insert_resource(CanvasImageBindGroups {
        bind_group: bind_group_0,
    });
}

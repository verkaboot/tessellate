use super::{
    pipeline::CanvasPipeline,
    sprite::{CanvasImages, MouseData},
};
use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets, render_resource::*, renderer::RenderDevice, texture::GpuImage,
    },
};

#[derive(Resource)]
pub struct CanvasImageBindGroups {
    pub bind_groups: [BindGroup; 2],
}

pub fn prepare(
    mut commands: Commands,
    pipeline: Res<CanvasPipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    canvas_images: Res<CanvasImages>,
    mouse_position: Res<MouseData>,
    render_device: Res<RenderDevice>,
) {
    let view_a = gpu_images.get(&canvas_images.texture_a).unwrap();
    let view_b = gpu_images.get(&canvas_images.texture_b).unwrap();

    let mouse_position_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&[mouse_position.pos]),
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
        &BindGroupEntries::sequential((
            &view_a.texture_view,
            &view_b.texture_view,
            mouse_position_binding.clone(),
        )),
    );
    let bind_group_1 = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((
            &view_b.texture_view,
            &view_a.texture_view,
            mouse_position_binding,
        )),
    );
    commands.insert_resource(CanvasImageBindGroups {
        bind_groups: [bind_group_0, bind_group_1],
    });
}

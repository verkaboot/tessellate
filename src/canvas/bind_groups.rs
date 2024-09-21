use super::{
    brush::{BrushColor, BrushSize},
    mouse::MouseData,
    pipeline::CanvasPipeline,
    sprite::CanvasImages,
};
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
    mouse_data: Res<MouseData>,
    brush_size: Res<BrushSize>,
    brush_color: Res<BrushColor>,
    render_device: Res<RenderDevice>,
) {
    let texture = gpu_images.get(&canvas_images.texture).unwrap();

    let mouse_pos_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&[mouse_data.world_pos]),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });

    let mouse_pos_binding = BufferBinding {
        buffer: &mouse_pos_buffer,
        offset: 0,
        size: None,
    };

    let brush_size_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&[brush_size.0]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let brush_size_binding = BufferBinding {
        buffer: &brush_size_buffer,
        offset: 0,
        size: None,
    };

    let brush_color_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&brush_color.to_srgba().to_f32_array()),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let brush_color_binding = BufferBinding {
        buffer: &brush_color_buffer,
        offset: 0,
        size: None,
    };

    let bind_group = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((
            &texture.texture_view,
            mouse_pos_binding,
            brush_size_binding,
            brush_color_binding,
        )),
    );

    commands.insert_resource(CanvasImageBindGroups { bind_group });
}

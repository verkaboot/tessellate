use crate::sprite::CanvasSprite;

use super::{
    brush::{BrushColor, BrushHardness, BrushSize},
    pipeline::CanvasPipeline,
    sprite::CanvasImages,
    tool::ToolData,
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
    mouse_data: Res<ToolData>,
    brush_size: Res<BrushSize>,
    brush_hardness: Res<BrushHardness>,
    brush_color: Res<BrushColor>,
    render_device: Res<RenderDevice>,
    canvas_sprite_q: Query<&CanvasSprite>,
) {
    let layered_texture = gpu_images.get(&canvas_images.layered_texture).unwrap();
    let sprite_image = gpu_images.get(&canvas_images.sprite_image).unwrap();

    let active_layer_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&[canvas_images.active_layer]),
        usage: BufferUsages::UNIFORM,
    });
    let active_layer_binding = BufferBinding {
        buffer: &active_layer_buffer,
        offset: 0,
        size: None,
    };

    let mouse_pos_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&[mouse_data.world_pos]),
        usage: BufferUsages::STORAGE,
    });
    let mouse_pos_binding = BufferBinding {
        buffer: &mouse_pos_buffer,
        offset: 0,
        size: None,
    };

    let brush_size_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&[brush_size.0]),
        usage: BufferUsages::UNIFORM,
    });
    let brush_size_binding = BufferBinding {
        buffer: &brush_size_buffer,
        offset: 0,
        size: None,
    };

    let brush_hardness_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&[brush_hardness.0]),
        usage: BufferUsages::UNIFORM,
    });
    let brush_hardness_binding = BufferBinding {
        buffer: &brush_hardness_buffer,
        offset: 0,
        size: None,
    };

    let brush_color_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&brush_color.to_srgba().to_f32_array()),
        usage: BufferUsages::UNIFORM,
    });
    let brush_color_binding = BufferBinding {
        buffer: &brush_color_buffer,
        offset: 0,
        size: None,
    };

    let canvas_transforms: Vec<Vec2> = canvas_sprite_q
        .iter()
        .map(|CanvasSprite(transform)| *transform)
        .collect();
    let contents = bytemuck::cast_slice(&canvas_transforms);
    let canvas_transforms_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        contents,
        usage: BufferUsages::STORAGE,
    });
    let canvas_transforms_binding = BufferBinding {
        buffer: &canvas_transforms_buffer,
        offset: 0,
        size: None,
    };

    let bind_group = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((
            &layered_texture.texture_view,
            &sprite_image.texture_view,
            active_layer_binding,
            mouse_pos_binding,
            brush_size_binding,
            brush_hardness_binding,
            brush_color_binding,
            canvas_transforms_binding,
        )),
    );

    commands.insert_resource(CanvasImageBindGroups { bind_group });
}

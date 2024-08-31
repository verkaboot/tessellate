use super::{pipeline::CanvasPipeline, sprite::CanvasImages};
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
    render_device: Res<RenderDevice>,
) {
    let view_a = gpu_images.get(&canvas_images.texture_a).unwrap();
    let view_b = gpu_images.get(&canvas_images.texture_b).unwrap();
    let bind_group_0 = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((&view_a.texture_view, &view_b.texture_view)),
    );
    let bind_group_1 = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((&view_b.texture_view, &view_a.texture_view)),
    );
    commands.insert_resource(CanvasImageBindGroups {
        bind_groups: [bind_group_0, bind_group_1],
    });
}

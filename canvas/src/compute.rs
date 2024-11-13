use std::f32::consts::E;

use crate::{brush::BrushHardness, sprite::CanvasSprite};

use super::{
    bind_groups::{self},
    brush::{BrushColor, BrushSize},
    mouse::ToolData,
    node::CanvasNode,
    pipeline::CanvasPipeline,
    sprite::CanvasImages,
};
use bevy::{
    prelude::*,
    render::{
        extract_component::ExtractComponentPlugin,
        extract_resource::ExtractResourcePlugin,
        render_graph::{RenderGraph, RenderLabel},
        Render, RenderApp, RenderSet,
    },
};

pub struct CanvasComputePlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct CanvasComputeLabel;

impl Plugin for CanvasComputePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractResourcePlugin::<CanvasImages>::default(),
            ExtractResourcePlugin::<ToolData>::default(),
            ExtractResourcePlugin::<BrushSize>::default(),
            ExtractResourcePlugin::<BrushHardness>::default(),
            ExtractResourcePlugin::<BrushColor>::default(),
            ExtractComponentPlugin::<CanvasSprite>::default(),
        ));

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            bind_groups::prepare.in_set(RenderSet::PrepareBindGroups),
        );

        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(CanvasComputeLabel, CanvasNode::default());
        render_graph.add_node_edge(CanvasComputeLabel, bevy::render::graph::CameraDriverLabel);
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<CanvasPipeline>();
    }
}

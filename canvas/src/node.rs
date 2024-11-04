use super::{
    bind_groups::CanvasImageBindGroups, mouse::ToolData, pipeline::CanvasPipeline,
    SHADER_ASSET_PATH, SIZE, WORKGROUP_SIZE,
};
use bevy::{
    prelude::*,
    render::{
        render_graph::{self},
        render_resource::*,
        renderer::RenderContext,
    },
};

pub struct CanvasNode {
    state: CanvasState,
}

enum CanvasState {
    Loading,
    Update,
}

impl Default for CanvasNode {
    fn default() -> Self {
        Self {
            state: CanvasState::Loading,
        }
    }
}

impl render_graph::Node for CanvasNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<CanvasPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            CanvasState::Loading => {
                match pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline) {
                    CachedPipelineState::Ok(_) => {
                        self.state = CanvasState::Update;
                    }
                    CachedPipelineState::Err(err) => {
                        panic!("Initializing assets/{SHADER_ASSET_PATH}:\n{err}")
                    }
                    _ => {}
                }
            }
            CanvasState::Update => (),
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        match self.state {
            CanvasState::Loading => {
                return Ok(());
            }
            CanvasState::Update => {}
        };

        let tool_data = world.resource::<ToolData>();
        if !tool_data.tool_active {
            return Ok(());
        };

        match tool_data.tool_type {
            crate::mouse::ToolType::Select => {}

            crate::mouse::ToolType::Paint => {
                let bind_group = &world.resource::<CanvasImageBindGroups>().bind_group;
                let pipeline_cache = world.resource::<PipelineCache>();
                let pipeline = world.resource::<CanvasPipeline>();

                let mut pass = render_context
                    .command_encoder()
                    .begin_compute_pass(&ComputePassDescriptor::default());

                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.paint_normal_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_group, &[]);
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }

            crate::mouse::ToolType::Erase => {
                let bind_group = &world.resource::<CanvasImageBindGroups>().bind_group;
                let pipeline_cache = world.resource::<PipelineCache>();
                let pipeline = world.resource::<CanvasPipeline>();

                let mut pass = render_context
                    .command_encoder()
                    .begin_compute_pass(&ComputePassDescriptor::default());

                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.paint_erase_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_group, &[]);
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
        }

        Ok(())
    }
}

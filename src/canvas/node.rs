use super::{
    bind_groups::CanvasImageBindGroups, pipeline::CanvasPipeline, SHADER_ASSET_PATH, SIZE,
    WORKGROUP_SIZE,
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
    Init,
    Update(usize),
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
                        self.state = CanvasState::Init;
                    }
                    CachedPipelineState::Err(err) => {
                        panic!("Initializing assets/{SHADER_ASSET_PATH}:\n{err}")
                    }
                    _ => {}
                }
            }
            CanvasState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = CanvasState::Update(1);
                }
            }
            CanvasState::Update(0) => {
                self.state = CanvasState::Update(1);
            }
            CanvasState::Update(1) => {
                self.state = CanvasState::Update(0);
            }
            CanvasState::Update(_) => unreachable!(),
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let bind_groups = &world.resource::<CanvasImageBindGroups>().bind_groups;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<CanvasPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        // select the pipeline based on the current state
        match self.state {
            CanvasState::Loading => {}
            CanvasState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups[0], &[]);
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
            CanvasState::Update(index) => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups[index], &[]);
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
        }

        Ok(())
    }
}
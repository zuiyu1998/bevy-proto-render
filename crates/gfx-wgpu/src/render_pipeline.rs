use frame_graph::prelude::*;

pub struct WgpuRenderPipeline {
    pub render_pipeline: wgpu::RenderPipeline,
}

impl RenderPipelineTrait for WgpuRenderPipeline {}

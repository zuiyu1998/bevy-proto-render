use crate::gfx_base::RenderPipelineTrait;

pub struct WgpuRenderPipeline {
    pub render_pipeline: wgpu::RenderPipeline,
}

impl RenderPipelineTrait for WgpuRenderPipeline {}

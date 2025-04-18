use downcast_rs::Downcast;

use crate::define_gfx_type;

use super::{RenderPipeline, RenderPipelineInfo};

pub trait PipelineCacheTrait: 'static + Sync + Send {
    fn get_render_pipeline(&self, id: &CachedRenderPipelineId) -> Option<&CachedPipeline>;
}

pub trait ErasedPipelineCacheTrait: 'static + Sync + Send + Downcast {
    fn get_render_pipeline(&self, id: &CachedRenderPipelineId) -> Option<&CachedPipeline>;
}

impl<T: PipelineCacheTrait> ErasedPipelineCacheTrait for T {
    fn get_render_pipeline(&self, id: &CachedRenderPipelineId) -> Option<&CachedPipeline> {
        <T as PipelineCacheTrait>::get_render_pipeline(self, id)
    }
}

define_gfx_type!(PipelineCache, PipelineCacheTrait, ErasedPipelineCacheTrait);

impl PipelineCache {
    pub fn get_render_pipeline(&self, id: &CachedRenderPipelineId) -> Option<&CachedPipeline> {
        self.value.get_render_pipeline(id)
    }
}

pub enum PipelineInfo {
    RenderPipelineInfo(Box<RenderPipelineInfo>),
}

pub enum Pipeline {
    RenderPipeline(RenderPipeline),
}

pub enum CachedPipelineState {
    Ok(Pipeline),
    Queued,
}

pub struct CachedPipeline {
    pub desc: PipelineInfo,
    state: CachedPipelineState,
}

impl CachedPipeline {
    pub fn get_render_pipeline(&self) -> Option<&RenderPipeline> {
        if let CachedPipelineState::Ok(Pipeline::RenderPipeline(ref pipeline)) = self.state {
            Some(pipeline)
        } else {
            None
        }
    }
}

pub struct CachedRenderPipelineId(usize);

impl CachedRenderPipelineId {
    pub const INVALID: Self = CachedRenderPipelineId(usize::MAX);

    #[inline]
    pub fn id(&self) -> usize {
        self.0
    }
}

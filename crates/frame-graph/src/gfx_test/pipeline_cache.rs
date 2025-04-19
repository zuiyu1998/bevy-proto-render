use crate::gfx_base::*;

pub struct TestPipelineCache;

impl PipelineCacheTrait for TestPipelineCache {
    fn get_render_pipeline(&self, _id: &CachedRenderPipelineId) -> Option<&CachedPipeline> {
        None
    }
}

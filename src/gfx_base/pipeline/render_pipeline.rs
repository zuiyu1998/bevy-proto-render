use downcast_rs::Downcast;

use crate::define_gfx_type;

pub trait RenderPipelineTrait: 'static + Sync + Send {}

pub trait ErasedRenderPipelineTrait: 'static + Sync + Send + Downcast {}

impl<T: RenderPipelineTrait> ErasedRenderPipelineTrait for T {}

define_gfx_type!(
    RenderPipeline,
    RenderPipelineTrait,
    ErasedRenderPipelineTrait
);

pub struct RenderPipelineInfo {}

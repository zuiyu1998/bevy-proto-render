use downcast_rs::Downcast;

use crate::{ResourceTable, Result, define_gfx_type};

pub trait CommandBufferTrait: 'static + Sync + Send {
    fn begin_render_pass(
        &mut self,
        resource_table: &ResourceTable,
        render_pass_info: &RenderPassInfo,
    ) -> Result<()>;

    fn end_render_pass(&mut self);
}

pub trait ErasedCommandBufferTrait: 'static + Sync + Send + Downcast {
    fn begin_render_pass(
        &mut self,
        resource_table: &ResourceTable,
        render_pass_info: &RenderPassInfo,
    ) -> Result<()>;

    fn end_render_pass(&mut self);
}

impl<T: CommandBufferTrait> ErasedCommandBufferTrait for T {
    fn begin_render_pass(
        &mut self,
        resource_table: &ResourceTable,
        render_pass_info: &RenderPassInfo,
    ) -> Result<()> {
        <T as CommandBufferTrait>::begin_render_pass(self, resource_table, render_pass_info)
    }

    fn end_render_pass(&mut self) {
        <T as CommandBufferTrait>::end_render_pass(self);
    }
}

define_gfx_type!(CommandBuffer, CommandBufferTrait, ErasedCommandBufferTrait);

#[derive(Clone)]
pub struct RenderPassInfo {}

impl CommandBuffer {
    pub fn begin_render_pass(
        &mut self,
        resource_table: &ResourceTable,
        render_pass_info: &RenderPassInfo,
    ) -> Result<()> {
        self.value
            .begin_render_pass(resource_table, render_pass_info)
    }

    pub fn end_render_pass(&mut self) {
        self.value.end_render_pass();
    }
}

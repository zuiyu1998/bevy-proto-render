use std::ops::Range;

use downcast_rs::Downcast;

use crate::{ResourceTable, Result, define_gfx_type};

pub trait CommandBufferTrait: 'static + Sync + Send {
    fn begin_render_pass(
        &mut self,
        resource_table: &ResourceTable,
        render_pass_info: &RenderPassInfo,
    ) -> Result<()>;

    fn end_render_pass(&mut self);

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);
}

pub trait ErasedCommandBufferTrait: 'static + Sync + Send + Downcast {
    fn begin_render_pass(
        &mut self,
        resource_table: &ResourceTable,
        render_pass_info: &RenderPassInfo,
    ) -> Result<()>;

    fn end_render_pass(&mut self);

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);
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

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        <T as CommandBufferTrait>::draw(self, vertices, instances);
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

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.value.draw(vertices, instances);
    }
}

use std::ops::Range;

use downcast_rs::Downcast;

use crate::{
    Result, define_gfx_type,
    frame_graph::{GpuRead, ResourceRef, ResourceTable},
};

use super::{BindGroupRef, Buffer, RenderPipeline};

pub trait CommandBufferTrait: 'static + Sync + Send {
    fn begin_render_pass(
        &mut self,
        resource_table: &ResourceTable,
        render_pass_info: &RenderPassInfo,
    ) -> Result<()>;

    fn end_render_pass(&mut self);

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);

    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>);

    fn set_pipeline(&mut self, pipeline: &RenderPipeline);

    fn set_bind_group(
        &mut self,
        resource_table: &ResourceTable,
        bind_group_ref: Option<&BindGroupRef>,
        index: u32,
        offsets: &[u32],
    ) -> Result<()>;

    fn set_vertex_buffer(
        &mut self,
        resource_table: &ResourceTable,
        buffer_ref: &ResourceRef<Buffer, GpuRead>,
        slot: u32,
    ) -> Result<()>;
}

pub trait ErasedCommandBufferTrait: 'static + Sync + Send + Downcast {
    fn begin_render_pass(
        &mut self,
        resource_table: &ResourceTable,
        render_pass_info: &RenderPassInfo,
    ) -> Result<()>;

    fn end_render_pass(&mut self);

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);

    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>);

    fn set_pipeline(&mut self, pipeline: &RenderPipeline);

    fn set_bind_group(
        &mut self,
        resource_table: &ResourceTable,
        bind_group_ref: Option<&BindGroupRef>,
        index: u32,
        offsets: &[u32],
    ) -> Result<()>;

    fn set_vertex_buffer(
        &mut self,
        resource_table: &ResourceTable,
        buffer_ref: &ResourceRef<Buffer, GpuRead>,
        slot: u32,
    ) -> Result<()>;
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

    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        <T as CommandBufferTrait>::draw_indexed(self, indices, base_vertex, instances);
    }

    fn set_pipeline(&mut self, pipeline: &RenderPipeline) {
        <T as CommandBufferTrait>::set_pipeline(self, pipeline);
    }

    fn set_bind_group(
        &mut self,
        resource_table: &ResourceTable,
        bind_group_ref: Option<&BindGroupRef>,
        index: u32,
        offsets: &[u32],
    ) -> Result<()> {
        <T as CommandBufferTrait>::set_bind_group(
            self,
            resource_table,
            bind_group_ref,
            index,
            offsets,
        )
    }

    fn set_vertex_buffer(
        &mut self,
        resource_table: &ResourceTable,
        buffer_ref: &ResourceRef<Buffer, GpuRead>,
        slot: u32,
    ) -> Result<()> {
        <T as CommandBufferTrait>::set_vertex_buffer(self, resource_table, buffer_ref, slot)
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

    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.value.draw_indexed(indices, base_vertex, instances);
    }

    pub fn set_pipeline(&mut self, pipeline: &RenderPipeline) {
        self.value.set_pipeline(pipeline);
    }

    pub fn set_bind_group(
        &mut self,
        resource_table: &ResourceTable,
        bind_group_ref: Option<&BindGroupRef>,
        index: u32,
        offsets: &[u32],
    ) -> Result<()> {
        self.value
            .set_bind_group(resource_table, bind_group_ref, index, offsets)
    }

    pub fn set_vertex_buffer(
        &mut self,
        resource_table: &ResourceTable,
        buffer_ref: &ResourceRef<Buffer, GpuRead>,
        slot: u32,
    ) -> Result<()> {
        self.value
            .set_vertex_buffer(resource_table, buffer_ref, slot)
    }
}

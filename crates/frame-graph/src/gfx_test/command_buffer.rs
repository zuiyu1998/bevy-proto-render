use std::ops::Range;

use crate::{Result, frame_graph::*, gfx_base::*};

#[derive(Default)]
pub struct TestCommandBuffer {
    render_pass_info: Option<RenderPassInfo>,
}

impl CommandBufferTrait for TestCommandBuffer {
    fn begin_render_pass(
        &mut self,
        _resource_table: &ResourceTable,
        render_pass_info: &RenderPassInfo,
    ) -> Result<()> {
        self.render_pass_info = Some(render_pass_info.clone());

        Ok(())
    }

    fn end_render_pass(&mut self) {}

    fn draw(&mut self, _vertices: Range<u32>, _instances: Range<u32>) {}

    fn draw_indexed(&mut self, _indices: Range<u32>, _base_vertex: i32, _instances: Range<u32>) {}

    fn set_pipeline(&mut self, _pipeline: &RenderPipeline) {}

    fn set_bind_group(
        &mut self,
        _resource_table: &ResourceTable,
        _bind_group_ref: Option<&BindGroupRef>,
        _index: u32,
        _offsets: &[u32],
    ) -> Result<()> {
        Ok(())
    }

    fn set_vertex_buffer(
        &mut self,
        _resource_table: &ResourceTable,
        _buffer_ref: &ResourceRef<Buffer, GpuRead>,
        _slot: u32,
    ) -> Result<()> {
        Ok(())
    }

    fn set_index_buffer(
        &mut self,
        _resource_table: &ResourceTable,
        _buffer_ref: &ResourceRef<Buffer, GpuRead>,
        _index_format: IndexFormat,
    ) -> Result<()> {
        Ok(())
    }
}

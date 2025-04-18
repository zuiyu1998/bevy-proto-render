use std::{ops::Range, sync::Arc};

use crate::{
    ErrorKind, Result,
    gfx_base::{
        BindGroupRef, CachedRenderPipelineId, CommandBuffer, PipelineCache, RenderDevice,
        RenderPassInfo,
    },
};

use super::ResourceTable;

pub struct RenderContext<'a> {
    device: &'a Arc<RenderDevice>,
    command_buffer: Option<CommandBuffer>,
    command_buffer_queue: Vec<CommandBuffer>,
    resource_table: ResourceTable,
    pipeline_cache: &'a PipelineCache,
}

impl<'a> RenderContext<'a> {
    pub fn new(device: &'a Arc<RenderDevice>, pipeline_cache: &'a PipelineCache) -> Self {
        Self {
            device,
            command_buffer: None,
            command_buffer_queue: vec![],
            resource_table: Default::default(),
            pipeline_cache,
        }
    }

    pub fn set_bind_group(
        &mut self,
        bind_group_ref: Option<&BindGroupRef>,
        index: u32,
        offsets: &[u32],
    ) -> Result<()> {
        self.command_buffer.as_mut().unwrap().set_bind_group(
            &self.resource_table,
            bind_group_ref,
            index,
            offsets,
        )
    }

    pub fn set_pipeline(&mut self, id: &CachedRenderPipelineId) -> Result<()> {
        let cache_pipeline = self
            .pipeline_cache
            .get_render_pipeline(id)
            .ok_or(ErrorKind::RenderPipelineNotFound)?;

        let pipeline = cache_pipeline
            .get_render_pipeline()
            .ok_or(ErrorKind::PipelineNotMatch)?;

        self.command_buffer.as_mut().unwrap().set_pipeline(pipeline);

        Ok(())
    }

    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.command_buffer
            .as_mut()
            .unwrap()
            .draw_indexed(indices, base_vertex, instances);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.command_buffer
            .as_mut()
            .unwrap()
            .draw(vertices, instances);
    }

    pub fn begin_render_pass(&mut self, render_pass_info: &RenderPassInfo) -> Result<()> {
        self.flush();

        let mut command_buffer = self.device.create_command_buffer();

        command_buffer.begin_render_pass(&self.resource_table, render_pass_info)?;

        self.command_buffer = Some(command_buffer);

        Ok(())
    }

    pub fn flush(&mut self) {
        if let Some(mut command_buffer) = self.command_buffer.take() {
            command_buffer.end_render_pass();

            self.command_buffer_queue.push(command_buffer);
        }
    }
}

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use crate::{gfx_base::*, gfx_test::*};

    use super::RenderContext;

    #[test]
    fn test_render_context() {
        let device = Arc::new(RenderDevice::new(TestRenderDevice));

        let pipeline_cache = PipelineCache::new(TestPipelineCache);

        let mut render_context = RenderContext::new(&device, &pipeline_cache);

        let render_pass_info = RenderPassInfo {};

        render_context.begin_render_pass(&render_pass_info).unwrap();

        assert!(render_context.command_buffer.is_some());

        render_context.flush();

        assert_eq!(render_context.command_buffer_queue.len(), 1);
    }
}

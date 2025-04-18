use std::{ops::Range, sync::Arc};

use crate::{CommandBuffer, ErrorKind, RenderDevice, RenderPassInfo, Result};

use super::ResourceTable;

pub struct RenderContext<'a> {
    device: &'a Arc<RenderDevice>,
    command_buffer: Option<CommandBuffer>,
    command_buffer_queue: Vec<CommandBuffer>,
    resource_table: ResourceTable,
}

impl<'a> RenderContext<'a> {
    pub fn new(device: &'a Arc<RenderDevice>) -> Self {
        Self {
            device,
            command_buffer: None,
            command_buffer_queue: vec![],
            resource_table: Default::default(),
        }
    }

    pub fn draw_indexed(
        &mut self,
        indices: Range<u32>,
        base_vertex: i32,
        instances: Range<u32>,
    ) -> Result<()> {
        if self.command_buffer.is_none() {
            return Err(ErrorKind::CommandBufferNotFound.into());
        }

        self.command_buffer
            .as_mut()
            .unwrap()
            .draw_indexed(indices, base_vertex, instances);

        Ok(())
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) -> Result<()> {
        if self.command_buffer.is_none() {
            return Err(ErrorKind::CommandBufferNotFound.into());
        }

        self.command_buffer
            .as_mut()
            .unwrap()
            .draw(vertices, instances);

        Ok(())
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

    use crate::{RenderDevice, RenderPassInfo, gfx_test::*};

    use super::RenderContext;

    #[test]
    fn test_render_context() {
        let device = Arc::new(RenderDevice::new(TestRenderDevice));

        let mut render_context = RenderContext::new(&device);

        let render_pass_info = RenderPassInfo {};

        render_context.begin_render_pass(&render_pass_info).unwrap();

        assert!(render_context.command_buffer.is_some());

        render_context.flush();

        assert_eq!(render_context.command_buffer_queue.len(), 1);
    }
}

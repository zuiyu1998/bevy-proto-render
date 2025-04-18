use std::ops::Range;

use crate::{Result, frame_graph::*, gfx_base::*};

pub struct WgpuCommandBuffer {
    device: wgpu::Device,
    command_encoder: Option<wgpu::CommandEncoder>,
    render_pass: Option<wgpu::RenderPass<'static>>,
    command_buffer: Option<wgpu::CommandBuffer>,
}

impl WgpuCommandBuffer {
    pub fn new(device: wgpu::Device) -> Self {
        WgpuCommandBuffer {
            device,
            command_encoder: None,
            render_pass: None,
            command_buffer: None,
        }
    }
}

impl CommandBufferTrait for WgpuCommandBuffer {
    fn begin_render_pass(
        &mut self,
        _resource_table: &ResourceTable,
        _render_pass_info: &RenderPassInfo,
    ) -> Result<()> {
        let mut command_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[],
            ..Default::default()
        });

        let render_pass = render_pass.forget_lifetime();
        self.render_pass = Some(render_pass);

        self.command_encoder = Some(command_encoder);

        Ok(())
    }

    fn end_render_pass(&mut self) {
        if let Some(render_pass) = self.render_pass.take() {
            drop(render_pass);
        }

        if let Some(command_encoder) = self.command_encoder.take() {
            let command_buffer = command_encoder.finish();
            self.command_buffer = Some(command_buffer);
        }
    }

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        assert!(self.command_encoder.is_some());
        self.render_pass.as_mut().unwrap().draw(vertices, instances);
    }
}

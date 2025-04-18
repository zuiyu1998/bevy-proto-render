use std::ops::Range;

use crate::{ErrorKind, Result, frame_graph::*, gfx_base::*};

use super::{WgpuBindGroupInfo, WgpuBuffer, WgpuRenderPipeline};

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
        self.render_pass.as_mut().unwrap().draw(vertices, instances);
    }

    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.render_pass
            .as_mut()
            .unwrap()
            .draw_indexed(indices, base_vertex, instances);
    }

    fn set_pipeline(&mut self, pipeline: &RenderPipeline) {
        let pipeline = &pipeline
            .downcast_ref::<WgpuRenderPipeline>()
            .unwrap()
            .render_pipeline;

        self.render_pass.as_mut().unwrap().set_pipeline(pipeline);
    }

    fn set_bind_group(
        &mut self,
        resource_table: &ResourceTable,
        bind_group_ref: Option<&BindGroupRef>,
        index: u32,
        offsets: &[u32],
    ) -> Result<()> {
        if bind_group_ref.is_none() {
            self.render_pass
                .as_mut()
                .unwrap()
                .set_bind_group(index, None, offsets);

            return Ok(());
        }

        let bind_group_ref = bind_group_ref.unwrap();

        let bind_group_info = WgpuBindGroupInfo::extract(bind_group_ref, resource_table)?;

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: bind_group_info.layout,
            entries: bind_group_info.entries,
        });

        self.render_pass
            .as_mut()
            .unwrap()
            .set_bind_group(index, Some(&bind_group), offsets);

        Ok(())
    }

    fn set_vertex_buffer(
        &mut self,
        resource_table: &ResourceTable,
        buffer_ref: &ResourceRef<Buffer, GpuRead>,
        slot: u32,
    ) -> Result<()> {
        let buffer = resource_table
            .get_resource(buffer_ref)
            .ok_or(ErrorKind::ResourceNotFound)?;

        let wgpu_buffer = buffer.downcast_ref::<WgpuBuffer>().unwrap();

        self.render_pass
            .as_mut()
            .unwrap()
            .set_vertex_buffer(slot, wgpu_buffer.buffer.slice(0..));

        Ok(())
    }
}

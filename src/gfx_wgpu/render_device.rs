use crate::gfx_base::*;

pub struct WgpuRenderDevice {
    pub value: wgpu::Device,
}

impl RenderDeviceTrait for WgpuRenderDevice {
    fn create_command_buffer(&self) -> CommandBuffer {
        todo!()
    }
}

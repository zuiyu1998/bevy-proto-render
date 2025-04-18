use crate::gfx_base::BufferTrait;

pub struct WgpuBuffer {
    pub buffer: wgpu::Buffer,
}

impl BufferTrait for WgpuBuffer {}

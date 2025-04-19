use frame_graph::prelude::*;

pub struct WgpuBuffer {
    pub buffer: wgpu::Buffer,
}

impl BufferTrait for WgpuBuffer {}

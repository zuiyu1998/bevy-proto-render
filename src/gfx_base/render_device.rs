use crate::define_gfx_type;
use downcast_rs::Downcast;

use super::CommandBuffer;

pub trait RenderDeviceTrait: 'static + Sync + Send {
    fn create_command_buffer(&self) -> CommandBuffer;
}

pub trait ErasedRenderDeviceTrait: 'static + Sync + Send + Downcast {
    fn create_command_buffer(&self) -> CommandBuffer;
}

impl<T: RenderDeviceTrait> ErasedRenderDeviceTrait for T {
    fn create_command_buffer(&self) -> CommandBuffer {
        <T as RenderDeviceTrait>::create_command_buffer(self)
    }
}

define_gfx_type!(RenderDevice, RenderDeviceTrait, ErasedRenderDeviceTrait);

impl RenderDevice {
    pub fn create_command_buffer(&self) -> CommandBuffer {
        self.value.create_command_buffer()
    }
}

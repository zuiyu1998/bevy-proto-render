use crate::define_gfx_type;
use downcast_rs::Downcast;

use super::CommandBuffer;

pub trait RenderDeviceTrait: 'static + Sync + Send + Clone {
    fn create_command_buffer(&self) -> CommandBuffer;
}

pub trait ErasedRenderDeviceTrait: 'static + Sync + Send + Downcast {
    fn create_command_buffer(&self) -> CommandBuffer;

    fn clone_value(&self) -> Box<dyn ErasedRenderDeviceTrait>;
}

impl<T: RenderDeviceTrait> ErasedRenderDeviceTrait for T {
    fn create_command_buffer(&self) -> CommandBuffer {
        <T as RenderDeviceTrait>::create_command_buffer(self)
    }

    fn clone_value(&self) -> Box<dyn ErasedRenderDeviceTrait> {
        Box::new(self.clone())
    }
}

define_gfx_type!(RenderDevice, RenderDeviceTrait, ErasedRenderDeviceTrait);

impl Clone for RenderDevice {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone_value(),
        }
    }
}

impl RenderDevice {
    pub fn create_command_buffer(&self) -> CommandBuffer {
        self.value.create_command_buffer()
    }
}

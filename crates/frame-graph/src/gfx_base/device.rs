use crate::define_gfx_type;
use downcast_rs::Downcast;

use super::CommandBuffer;

pub trait DeviceTrait: 'static + Sync + Send + Clone {
    fn create_command_buffer(&self) -> CommandBuffer;
}

pub trait ErasedDeviceTrait: 'static + Sync + Send + Downcast {
    fn create_command_buffer(&self) -> CommandBuffer;

    fn clone_value(&self) -> Box<dyn ErasedDeviceTrait>;
}

impl<T: DeviceTrait> ErasedDeviceTrait for T {
    fn create_command_buffer(&self) -> CommandBuffer {
        <T as DeviceTrait>::create_command_buffer(self)
    }

    fn clone_value(&self) -> Box<dyn ErasedDeviceTrait> {
        Box::new(self.clone())
    }
}

define_gfx_type!(Device, DeviceTrait, ErasedDeviceTrait);

impl Clone for Device {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone_value(),
        }
    }
}

impl Device {
    pub fn create_command_buffer(&self) -> CommandBuffer {
        self.value.create_command_buffer()
    }
}

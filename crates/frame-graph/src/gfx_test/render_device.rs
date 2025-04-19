use crate::gfx_base::{CommandBuffer, DeviceTrait};

use super::TestCommandBuffer;

#[derive(Clone)]
pub struct TestRenderDevice;

impl DeviceTrait for TestRenderDevice {
    fn create_command_buffer(&self) -> CommandBuffer {
        CommandBuffer::new(TestCommandBuffer::default())
    }
}

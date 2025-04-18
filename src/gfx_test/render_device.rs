use crate::gfx_base::{CommandBuffer, RenderDeviceTrait};

use super::TestCommandBuffer;

pub struct TestRenderDevice;

impl RenderDeviceTrait for TestRenderDevice {
    fn create_command_buffer(&self) -> CommandBuffer {
        CommandBuffer::new(TestCommandBuffer::default())
    }
}

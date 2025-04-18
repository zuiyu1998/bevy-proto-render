use crate::{CommandBufferTrait, RenderPassInfo, ResourceTable};

#[derive(Default)]
pub struct TestCommandBuffer {
    render_pass_info: Option<RenderPassInfo>,
}

impl CommandBufferTrait for TestCommandBuffer {
    fn begin_render_pass(
        &mut self,
        _resource_table: &ResourceTable,
        render_pass_info: &RenderPassInfo,
    ) -> crate::Result<()> {
        self.render_pass_info = Some(render_pass_info.clone());

        Ok(())
    }

    fn end_render_pass(&mut self) {}
}

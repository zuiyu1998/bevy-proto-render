use super::{FrameGraph, PassNode, RenderContext};
use crate::{Result, gfx_base::TypeHandle};

#[derive(Default)]
pub struct DevicePass {}

impl DevicePass {
    pub fn execute(&mut self, _render_context: &mut RenderContext) -> Result<()> {
        Ok(())
    }

    pub fn extra(&mut self, _graph: &mut FrameGraph, _handle: TypeHandle<PassNode>) {}
}

use crate::{
    Result,
    frame_graph::{GetView, ResourceTable},
    gfx_base::{BindGroupRef, BindGroupTrait},
};

pub struct WgpuBindGroup {
    pub bind_group: wgpu::BindGroup,
}

impl BindGroupTrait for WgpuBindGroup {}

pub struct WgpuBindGroupView<'a> {
    pub layout: &'a wgpu::BindGroupLayout,
    pub entries: &'a [wgpu::BindGroupEntry<'a>],
}

impl<'a> GetView<'a> for ResourceTable {
    type ViewRef = BindGroupRef;
    type View = WgpuBindGroupView<'a>;

    fn get_view(&self, _view_ref: &Self::ViewRef) -> Result<WgpuBindGroupView<'a>> {
        todo!()
    }
}

use frame_graph::prelude::*;

pub struct WgpuBindGroup {
    pub bind_group: wgpu::BindGroup,
}

impl BindGroupTrait for WgpuBindGroup {}

pub struct WgpuBindGroupView<'a> {
    pub layout: &'a wgpu::BindGroupLayout,
    pub entries: &'a [wgpu::BindGroupEntry<'a>],
}

impl ResourceView for WgpuBindGroupView<'_> {
    type ViewRef = BindGroupRef;

    fn prepare_view(_resource_table: &ResourceTable, _view_ref: &Self::ViewRef) -> Result<Self> {
        todo!()
    }
}

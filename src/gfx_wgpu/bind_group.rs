use crate::{
    Result,
    frame_graph::{ExtractResourceTable, ResourceTable},
    gfx_base::{BindGroupRef, BindGroupTrait},
};

pub struct WgpuBindGroup {
    pub bind_group: wgpu::BindGroup,
}

impl BindGroupTrait for WgpuBindGroup {}

pub struct WgpuBindGroupInfo<'a> {
    pub layout: &'a wgpu::BindGroupLayout,
    pub entries: &'a [wgpu::BindGroupEntry<'a>],
}

impl ExtractResourceTable for WgpuBindGroupInfo<'_> {
    type Source = BindGroupRef;

    fn extract(_source: &Self::Source, _resource_table: &ResourceTable) -> Result<Self> {
        todo!()
    }
}

use crate::Result;

use super::{GpuRead, ResourceRef};

#[derive(Default)]
pub struct ResourceTable;

impl ResourceTable {
    pub fn get_resource<ResourceType>(
        &self,
        _resource_ref: &ResourceRef<ResourceType, GpuRead>,
    ) -> Option<&ResourceType> {
        todo!()
    }
}

pub trait ExtractResourceTable: Sized {
    type Source: 'static;

    fn extract(source: &Self::Source, resource_table: &ResourceTable) -> Result<Self>;
}

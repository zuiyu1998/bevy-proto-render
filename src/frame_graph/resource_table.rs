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

pub trait GetView<'a>: Sized {
    type ViewRef;
    type View: 'a;

    fn get_view(&self, view_ref: &Self::ViewRef) -> Result<Self::View>;
}

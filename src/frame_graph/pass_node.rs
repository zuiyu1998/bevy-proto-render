use crate::gfx_base::TypeHandle;

use super::{ResourceNode, VirtualResource};

pub struct PassNode {
    pub name: String,
    pub handle: TypeHandle<PassNode>,
    pub writes: Vec<TypeHandle<ResourceNode>>,
    pub reads: Vec<TypeHandle<ResourceNode>>,
    pub resource_request_array: Vec<TypeHandle<VirtualResource>>,
    pub resource_release_array: Vec<TypeHandle<VirtualResource>>,
}

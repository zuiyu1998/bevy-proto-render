use std::marker::PhantomData;

use crate::gfx_base::TypeHandle;

use super::PassNode;

#[derive(Clone)]
pub enum ImportedVirtualResource {}

#[derive(Clone)]
pub struct VirtualResource {
    pub state: ResourceState,
    pub info: ResourceInfo,
}

#[derive(Clone)]
pub struct ResourceInfo {
    pub name: String,
    pub handle: TypeHandle<VirtualResource>,
    version: u32,
    pub first_use_pass: Option<TypeHandle<PassNode>>,
    pub last_user_pass: Option<TypeHandle<PassNode>>,
}

impl ResourceInfo {
    pub fn new(name: &str, handle: TypeHandle<VirtualResource>) -> Self {
        ResourceInfo {
            name: name.to_string(),
            handle,
            version: 0,
            first_use_pass: None,
            last_user_pass: None,
        }
    }
}

impl ResourceInfo {
    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn new_version(&mut self) {
        self.version += 1
    }

    pub fn update_lifetime(&mut self, handle: TypeHandle<PassNode>) {
        if self.first_use_pass.is_none() {
            self.first_use_pass = Some(handle);
        }

        self.last_user_pass = Some(handle)
    }
}

#[derive(Clone)]
pub struct ImportedResourceState {
    pub desc: AnyResourceDescriptor,
    pub resource: ImportedVirtualResource,
}

#[derive(Clone)]
pub enum ResourceState {
    Setuped(AnyResourceDescriptor),
    Imported(ImportedResourceState),
}

#[derive(Clone)]
pub enum AnyResourceDescriptor {}

pub enum AnyResource {}

pub trait Resource: 'static {
    type Descriptor: ResourceDescriptor;

    fn borrow_resource(res: &AnyResource) -> &Self;

    fn get_desc(&self) -> &Self::Descriptor;
}

pub trait ResourceDescriptor: 'static + Clone + Into<AnyResourceDescriptor> {
    type Resource: Resource;
}

pub trait TypeEquals {
    type Other;
    fn same(value: Self) -> Self::Other;
}

impl<T: Sized> TypeEquals for T {
    type Other = Self;
    fn same(value: Self) -> Self::Other {
        value
    }
}

pub struct ResourceRef<ResourceType, ViewType> {
    _marker: PhantomData<(ResourceType, ViewType)>,
}

pub trait GpuView {}

pub struct GpuRead;

impl GpuView for GpuRead {}

pub struct GpuWrite;

impl GpuView for GpuWrite {}

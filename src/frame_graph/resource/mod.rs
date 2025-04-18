use std::marker::PhantomData;

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

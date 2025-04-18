use std::marker::PhantomData;

pub struct ResourceRef<ResourceType, ViewType> {
    _marker: PhantomData<(ResourceType, ViewType)>,
}

pub trait GpuView {}

pub struct GpuRead;

impl GpuView for GpuRead {}

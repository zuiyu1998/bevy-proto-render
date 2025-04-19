use crate::define_gfx_type;
use downcast_rs::Downcast;

use super::BindGroupLayout;

pub trait BindGroupTrait: 'static + Sync + Send {}

pub trait ErasedBindGroupTrait: 'static + Sync + Send + Downcast {}

impl<T: BindGroupTrait> ErasedBindGroupTrait for T {}

define_gfx_type!(BindGroup, BindGroupTrait, ErasedBindGroupTrait);

pub enum BindingResourceRef {}

pub struct BindGroupEntryRef {
    pub binding: u32,
    pub resource: BindingResourceRef,
}

pub struct BindGroupRef {
    pub layout: BindGroupLayout,
    pub entries: Vec<BindGroupEntryRef>,
}

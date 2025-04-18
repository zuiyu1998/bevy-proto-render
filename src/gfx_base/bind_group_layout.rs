use crate::define_gfx_type;
use downcast_rs::Downcast;

pub trait BindGroupLayoutTrait: 'static + Sync + Send + Clone {}

pub trait ErasedBindGroupLayoutTrait: 'static + Sync + Send + Downcast {
    fn clone_value(&self) -> Box<dyn ErasedBindGroupLayoutTrait>;
}

impl<T: BindGroupLayoutTrait> ErasedBindGroupLayoutTrait for T {
    fn clone_value(&self) -> Box<dyn ErasedBindGroupLayoutTrait> {
        Box::new(self.clone())
    }
}

define_gfx_type!(
    BindGroupLayout,
    BindGroupLayoutTrait,
    ErasedBindGroupLayoutTrait
);

impl Clone for BindGroupLayout {
    fn clone(&self) -> Self {
        BindGroupLayout {
            value: self.value.clone_value(),
        }
    }
}

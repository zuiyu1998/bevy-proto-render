use crate::define_gfx_type_with_desc;
use downcast_rs::Downcast;

pub trait BufferTrait: 'static + Sync + Send {}

pub trait ErasedBufferTrait: 'static + Sync + Send + Downcast {}

impl<T: BufferTrait> ErasedBufferTrait for T {}

define_gfx_type_with_desc!(Buffer, BufferInfo, BufferTrait, ErasedBufferTrait);

pub struct BufferInfo {}

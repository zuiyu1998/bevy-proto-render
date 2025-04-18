use crate::Result;

#[derive(Default)]
pub struct ResourceTable;

pub trait ExtractResourceTable: 'static + Sized {
    type Source: 'static;

    fn extract(source: &Self::Source) -> Result<Self>;
}

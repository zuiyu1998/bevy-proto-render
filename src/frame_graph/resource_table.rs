use crate::Result;

#[derive(Default)]
pub struct ResourceTable;

pub trait ExtractResourceTable: Sized {
    type Source: 'static;

    fn extract(source: &Self::Source, resource_table: &ResourceTable) -> Result<Self>;
}

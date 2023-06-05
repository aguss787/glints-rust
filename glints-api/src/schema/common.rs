use async_graphql::{OutputType, SimpleObject};

#[derive(SimpleObject, Debug)]
pub struct Paginated<T>
where
    T: OutputType + Send + Sync,
{
    pub data: Vec<T>,
    pub page_no: i64,
    pub page_size: i64,
    pub total: i64,
}

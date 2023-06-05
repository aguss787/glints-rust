pub mod errors;
mod models;

use crate::hubber::errors::HubberError;
use async_trait::async_trait;
use glints_infra::postgresql::AsyncPgConnectionPool;
use shaku::{Component, Interface};
use std::sync::Arc;

#[derive(Component)]
#[shaku(interface = HubberAPI)]
pub(crate) struct HubberService {
    #[shaku(inject)]
    db_connection_pool: Arc<AsyncPgConnectionPool>,
}

#[derive(Debug, Clone)]
pub struct Hubber {
    pub id: String,
    pub code: String,
    pub name: String,
}

pub struct PaginationOptions {
    pub size: i64,
    pub offset: i64,
}

impl Default for PaginationOptions {
    fn default() -> Self {
        PaginationOptions {
            size: 20,
            offset: 0,
        }
    }
}

#[async_trait]
pub trait HubberAPI: Interface {
    async fn list_hubber(&self, pagination_opts: &PaginationOptions) -> HubberResult<Vec<Hubber>>;
    async fn count_hubber(&self) -> HubberResult<i64>;
}

type HubberResult<T> = Result<T, HubberError>;

#[async_trait]
impl HubberAPI for HubberService {
    async fn list_hubber(&self, pagination_opts: &PaginationOptions) -> HubberResult<Vec<Hubber>> {
        let mut connection = self.db_connection_pool.get_connection().await?;

        Ok(models::Hubber::get_paged(
            &mut connection,
            pagination_opts.size,
            pagination_opts.offset,
        )
        .await?
        .into_iter()
        .map(|o| Hubber {
            id: o.id.to_string(),
            code: o.code,
            name: o.name,
        })
        .collect())
    }

    async fn count_hubber(&self) -> HubberResult<i64> {
        let mut connection = self.db_connection_pool.get_connection().await?;

        Ok(models::Hubber::count(&mut connection).await?)
    }
}

mod models;

use async_trait::async_trait;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use glints_infra::diesel_schema::hubbers::table as hubbers;
use glints_infra::postgresql::AsyncPgConnectionPool;
use shaku::{Component, Interface};
use std::sync::Arc;

#[derive(Component)]
#[shaku(interface = HubberAPI)]
pub struct HubberService {
    #[shaku(inject)]
    db_connection_pool: Arc<AsyncPgConnectionPool>,
}

#[derive(Debug, Clone)]
pub struct Hubber {
    pub id: String,
    pub code: String,
    pub name: String,
}

#[async_trait]
pub trait HubberAPI: Interface {
    async fn list_hubber(&self) -> Vec<Hubber>;
}

#[async_trait]
impl HubberAPI for HubberService {
    async fn list_hubber(&self) -> Vec<Hubber> {
        let mut connection = self.db_connection_pool.get().await.expect("todo");
        let result = hubbers
            .limit(10)
            .load::<models::Hubber>(&mut connection)
            .await
            .expect("todo");

        result
            .into_iter()
            .map(|i| Hubber {
                id: i.id.to_string(),
                code: i.code,
                name: i.name,
            })
            .collect()
    }
}

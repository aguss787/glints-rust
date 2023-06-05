use crate::errors::InfraError;
use crate::InfraResult;
use anyhow::anyhow;
use diesel_async::pooled_connection::mobc::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncConnection, AsyncPgConnection};
use glints_config::GlintsConfig;
use shaku::{Component, HasComponent, Module, ModuleBuildContext};
use std::sync::Arc;

pub struct AsyncPgConnectionPool {
    pool: Pool<AsyncPgConnection>,
}

impl AsyncPgConnectionPool {
    fn new(database_url: impl Into<String>) -> Self {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
        let pool = Pool::new(config);

        AsyncPgConnectionPool { pool }
    }
}

impl<M: Module + HasComponent<GlintsConfig>> Component<M> for AsyncPgConnectionPool {
    type Interface = AsyncPgConnectionPool;
    type Parameters = ();

    fn build(
        context: &mut ModuleBuildContext<M>,
        _params: Self::Parameters,
    ) -> Box<Self::Interface> {
        let config: Arc<GlintsConfig> = M::build_component(context);
        let database_url = &config.postgres.database_url;

        Box::new(AsyncPgConnectionPool::new(database_url))
    }
}

impl AsyncPgConnectionPool {
    pub async fn get_connection(
        &self,
    ) -> InfraResult<impl AsyncConnection<Backend = diesel::pg::Pg>> {
        self.pool
            .get()
            .await
            .map_err(|o| InfraError::DatabaseConnectionPoolError { source: anyhow!(o) })
    }
}

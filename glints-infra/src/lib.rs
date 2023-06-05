pub mod diesel_schema;
pub mod errors;
pub mod postgresql;

use crate::errors::InfraError;
use glints_config::{ConfigModule, GlintsConfig};
use shaku::module;

pub type InfraResult<T> = Result<T, InfraError>;

module! {
    pub InfraModule {
        components = [ postgresql::AsyncPgConnectionPool ],
        providers = [],

        use ConfigModule {
            components = [ GlintsConfig ],
            providers = [],
        }
    }
}

impl Default for InfraModule {
    fn default() -> Self {
        InfraModule::builder(ConfigModule::default().into()).build()
    }
}

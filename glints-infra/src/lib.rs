pub mod diesel_schema;
pub mod postgresql;

use glints_config::{ConfigModule, GlintsConfig};
use shaku::module;

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

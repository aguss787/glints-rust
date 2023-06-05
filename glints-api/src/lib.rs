mod errors;
mod graphql;
mod schema;

use glints_config::{ConfigModule, GlintsConfig};
use glints_managed_talent::hubber::HubberAPI;
use glints_managed_talent::ManagedTalentModule;
use shaku::module;

type GraphQLResult<T> = Result<T, errors::GraphQlErrorWrapper>;

pub use graphql::configure_actix;
pub use schema::build as build_schema;

module! {
    pub APIModule {
        components = [],
        providers = [],

        use ManagedTalentModule {
            components = [ dyn HubberAPI ],
            providers = [],
        },

        use ConfigModule {
            components = [ GlintsConfig ],
            providers = [],
        },
    }
}

impl Default for APIModule {
    fn default() -> Self {
        APIModule::builder(
            ManagedTalentModule::default().into(),
            ConfigModule::default().into(),
        )
        .build()
    }
}

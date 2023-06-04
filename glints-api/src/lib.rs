pub mod graphql;
pub mod schema;

use crate::schema::Query;
use actix_web::guard;
use actix_web::web::{resource, Data, ServiceConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use glints_config::{ConfigModule, GlintsConfig};
use glints_managed_talent::HubberAPI;
use glints_managed_talent::ManagedTalentModule;
use shaku::module;

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

pub fn configure_actix(
    schema: Schema<Query, EmptyMutation, EmptySubscription>,
) -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .app_data(Data::new(schema))
            .service(resource("/").guard(guard::Post()).to(graphql::index));

        #[cfg(feature = "graphql-playground")]
        config.service(
            resource("/")
                .guard(guard::Get())
                .to(graphql::graphql_playground),
        );
    }
}

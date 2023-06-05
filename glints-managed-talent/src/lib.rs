pub mod hubber;

use glints_infra::postgresql::AsyncPgConnectionPool;
use glints_infra::InfraModule;
use shaku::module;

module! {
    pub ManagedTalentModule {
        components = [hubber::HubberService],
        providers = [],

        use InfraModule {
            components = [AsyncPgConnectionPool],
            providers = [],
        }
    }
}

impl Default for ManagedTalentModule {
    fn default() -> Self {
        ManagedTalentModule::builder(InfraModule::default().into()).build()
    }
}

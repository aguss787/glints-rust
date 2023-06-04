use once_cell::sync::Lazy;
pub use schema::GlintsConfig;
use shaku::{module, Component, Module, ModuleBuildContext};
use std::collections::HashMap;
use std::sync::RwLock;
use std::thread::ThreadId;

mod schema;

// TODO: make this module only available during testing
pub mod tests;

static CONFIG_MAP: Lazy<RwLock<HashMap<ThreadId, GlintsConfig>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
static DEFAULT_CONFIG: Lazy<GlintsConfig> = Lazy::new(|| GlintsConfig::read());

impl<M: Module> Component<M> for GlintsConfig {
    type Interface = GlintsConfig;
    type Parameters = ();

    fn build(
        _context: &mut ModuleBuildContext<M>,
        _params: Self::Parameters,
    ) -> Box<Self::Interface> {
        let id = std::thread::current().id();
        let map = CONFIG_MAP.read().expect("unable to get lock");

        if let Some(config) = map.get(&id) {
            log::trace!("loading specific thread config for {:?}", id);
            return Box::new(config.clone());
        }

        log::trace!("loading default config for {:?}", id);
        Box::new(DEFAULT_CONFIG.clone())
    }
}

module! {
    pub ConfigModule {
        components = [ GlintsConfig ],
        providers = [],
    }
}

impl Default for ConfigModule {
    fn default() -> Self {
        ConfigModule::builder().build()
    }
}

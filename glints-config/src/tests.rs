use crate::{GlintsConfig, CONFIG_MAP, DEFAULT_CONFIG};

pub fn overwrite_config_for_current_thread<F>(f: F)
where
    F: FnOnce(&mut GlintsConfig) -> (),
{
    let mut map = CONFIG_MAP.write().expect("unable to get write lock");

    let entry = (*map).entry(std::thread::current().id());
    let config = entry.or_insert(DEFAULT_CONFIG.clone());

    f(config)
}

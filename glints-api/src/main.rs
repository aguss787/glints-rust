use actix_web::{App, HttpServer};
use glints_config::GlintsConfig;
use kv_log_macro::info;
use log::LevelFilter;

use glints_api::configure_actix;
use glints_api::APIModule;
use shaku::HasComponent;
use std::str::FromStr;
use std::sync::Arc;

fn initialize_logging(log_level: &str) {
    let logging_level = LevelFilter::from_str(log_level).expect("Invalid logging level");

    #[cfg(feature = "json-logger")]
    json_env_logger::builder()
        .filter_level(logging_level)
        .init();
    #[cfg(not(feature = "json-logger"))]
    env_logger::builder().filter_level(logging_level).init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api_module = APIModule::default();
    let config: Arc<GlintsConfig> = api_module.resolve();

    initialize_logging(&(config.logging.level));

    info!("listening in 0.0.0.0:{}", config.api_http_server.port);
    let schema = glints_api::build_schema(api_module);
    HttpServer::new(move || App::new().configure(configure_actix(schema.clone())))
        .bind(("0.0.0.0", config.api_http_server.port))?
        .run()
        .await
}

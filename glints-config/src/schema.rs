use config::{Config, Environment, File};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Deserialize, Clone)]
pub struct GlintsConfig {
    pub postgres: PostgresConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
    #[serde(default)]
    pub api_http_server: ApiHttpServerConfig,
}

#[derive(Deserialize, Clone)]
pub struct PostgresConfig {
    pub database_url: String,
}

#[derive(Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        LoggingConfig {
            level: "INFO".to_string(),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct ApiHttpServerConfig {
    pub port: u16,
}

impl Default for ApiHttpServerConfig {
    fn default() -> Self {
        ApiHttpServerConfig { port: 8080 }
    }
}

impl GlintsConfig {
    /// Read glints config from both file and env variable
    ///
    /// It will read config file from GLINTS_CONFIG_DIR/GLINTS_RUN_ENV.yaml
    /// Config value can be overridden using env variable with the following format:
    ///   - GLINTS_<key 1>.<key 2>.<key 3>...<key n>
    ///   - example: GLINTS_POSTGRES.DATABASE_URL
    pub(crate) fn read() -> Self {
        Self::read_with_source(None)
    }

    fn read_var_from_source(
        env_source: &Option<HashMap<String, String>>,
        key: &String,
    ) -> Option<String> {
        match env_source {
            Some(map) => map.get(key).cloned(),
            None => env::var(key).ok(),
        }
    }

    fn read_with_source(env_source: Option<HashMap<String, String>>) -> Self {
        let run_env = Self::read_var_from_source(&env_source, &"GLINTS_RUN_ENV".to_string())
            .unwrap_or("development".to_string());

        let config_path = Self::read_var_from_source(&env_source, &"GLINTS_CONFIG_DIR".to_string())
            .unwrap_or("config".into());

        log::trace!("building config object");

        Config::builder()
            .add_source(File::with_name(&format!(
                "{}/{}.yaml",
                config_path, run_env
            )))
            .add_source(
                Environment::with_prefix("GLINTS")
                    .prefix_separator("_")
                    .separator("__")
                    .source(env_source),
            )
            .build()
            .expect("unable to build config")
            .try_deserialize()
            .expect("unable to deserialize config")
    }
}

#[cfg(test)]
mod test {
    use crate::schema::GlintsConfig;
    use std::collections::HashMap;

    #[test]
    fn can_read_config() {
        GlintsConfig::read_with_source(Some({
            let mut map = HashMap::new();
            map.insert("GLINTS_CONFIG_DIR".to_string(), "../config".to_string());

            map
        }));
    }

    #[test]
    #[should_panic]
    fn panic_on_missing_config() {
        GlintsConfig::read_with_source(Some({
            let mut map = HashMap::new();
            map.insert("GLINTS_CONFIG_DIR".to_string(), "bogus".to_string());

            map
        }));
    }

    #[test]
    fn can_override_using_env() {
        let config = GlintsConfig::read_with_source(Some({
            let mut map = HashMap::new();
            map.insert(
                "GLINTS_POSTGRES__DATABASE_URL".to_string(),
                "overridden".to_string(),
            );
            map.insert("GLINTS_CONFIG_DIR".to_string(), "../config".to_string());

            map
        }));
        assert_eq!(config.postgres.database_url, "overridden");
    }
}

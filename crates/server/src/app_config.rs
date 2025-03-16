use schematic::{Config, ConfigLoader};
use serde::{Deserialize, Serialize};

use crate::AppResult;

#[derive(Debug, Clone, Deserialize, Serialize, Config)]
pub struct AppConfig {
    #[setting(default = "127.0.0.1", env = "HOST")]
    pub host: String,

    #[setting(default = "8080", env = "PORT")]
    pub port: String,

    #[setting]
    pub worker: WorkerConfig,

    #[setting]
    pub web: WebConfig,

    #[setting]
    pub database: DatabaseConfig,

    #[setting(default = true, env = "AUTO_MIGRATE")]
    pub auto_migrate: bool,
}

#[derive(Debug, Clone, Config, Deserialize, Serialize, PartialEq)]
pub struct DatabaseConfig {
    #[setting(default = "surrealkv://./data/db", env = "DATABASE_URL")]
    pub url: String,
    #[setting(env = "DATABASE_USERNAME")]
    pub username: Option<String>,
    #[setting(env = "DATABASE_PASSWORD")]
    pub password: Option<String>,
    #[setting(default = true, env = "DATABASE_DEFINE_NS")]
    pub define_ns: bool,
    #[setting(default = "default", env = "DATABASE_NS")]
    pub ns: String,
    #[setting(default = true, env = "DATABASE_DEFINE_DB")]
    pub define_db: bool,
    #[setting(default = "default", env = "DATABASE_DB")]
    pub db: String,
    #[setting(default = true, env = "DATABASE_AUTO_MIGRATE")]
    pub auto_migrate: bool,
}

#[derive(Debug, Clone, Config, Deserialize, Serialize, PartialEq)]
pub struct WorkerConfig {
    #[setting(default = true, env = "WORKER_ENABLED")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Config, Deserialize, Serialize, PartialEq)]
pub struct WebConfig {
    #[setting(default = true, env = "WEB_SERVER_ENABLED")]
    pub enabled: bool,
}

impl AppConfig {
    pub fn from_path(path: Option<&str>) -> AppResult<Self> {
        let config_path = match path {
            Some(p) => p.to_string(),
            None => std::env::var("CONFIG_PATH").unwrap_or_else(|_| "data/config.yml".to_string()),
        };

        let result = ConfigLoader::<AppConfig>::new()
            .file_optional(config_path)?
            .load()?;

        Ok(result.config)
    }
}

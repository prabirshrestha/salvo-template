use schematic::{Config, ConfigLoader};

#[derive(Debug, Config)]
pub struct AppConfig {
    #[setting(default = "127.0.0.1", env = "HOST")]
    pub host: String,

    #[setting(default = "8080", env = "PORT")]
    pub port: String,

    #[setting(nested)]
    pub database: DatabaseConfig,

    #[setting(default = true, env = "AUTO_MIGRATE")]
    pub auto_migrate: bool,
}

#[derive(Debug, Config)]
pub struct DatabaseConfig {
    #[setting(
        default = "postgresql://postgres:postgres@localhost?temporary=false&data_dir=.%2F.salvo-template%2Fpostgres",
        env = "DATABASE_URL"
    )]
    pub url: String,

    #[setting(default = true, env = "DATABASE_USE_EMBEDDED")]
    pub use_embedded: bool,

    #[setting(default = "salvo-template", env = "DATABASE_NAME")]
    pub db_name: String,
}

impl AppConfig {
    pub fn from_path() -> anyhow::Result<Self> {
        let result = ConfigLoader::<AppConfig>::new()
            .file_optional("config.yml")?
            .load()?;

        Ok(result.config)
    }
}

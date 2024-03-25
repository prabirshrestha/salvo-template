use schematic::{Config, ConfigLoader};

#[derive(Debug, Config)]
pub struct AppConfig {
    #[setting(default = "127.0.0.1", env = "HOST")]
    pub host: String,

    #[setting(default = "8080", env = "PORT")]
    pub port: String,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let result = ConfigLoader::<AppConfig>::new()
            .file_optional("config.yml")?
            .load()?;

        Ok(result.config)
    }
}

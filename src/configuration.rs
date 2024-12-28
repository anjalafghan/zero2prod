use config::{Config, ConfigError, File};
use std::path::Path;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let builder = Config::builder()
        .set_default("application_port", 8000)?
        .add_source(File::from(Path::new(
            "/Users/anjalafghan/RustroverProjects/zero2prod/src/config/configuration.yaml",
        )));
    let config = builder.build()?;
    config.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

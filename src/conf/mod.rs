
use std::fs;
use std::path::{Path};
use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct AppConfig {
    host: String,
    port: u16,
    directory: String,
    pub(crate) authentication: AuthConfig,
    pub(crate) database: DatabaseConfig,
    pub(crate) cache: CacheConfig,
    pub(crate) log: Option<LogConfig>,
    pub(crate) kubernetes: KubeConfig,
    pub(crate) custom: Option<CustomConfig>,
}

impl AppConfig {
    pub fn new(config: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config: AppConfig = toml::from_str(config)?;
        Ok(config)
    }

    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let config = fs::read_to_string(path)?;
        Self::new(&config)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthConfig {
    admin_user: Option<String>,
    admin_password: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub(crate) driver: String,
    pub(crate) url: String,
    socket: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CacheConfig {
    host: String,
    port: u16,
    namespace: Option<String>,
    username: Option<String>,
    password: Option<String>
}


#[derive(Deserialize, Debug, Clone)]
pub struct LogConfig {
    pub(crate) access_log: Option<String>,
    pub(crate) error_log: Option<String>,
    pub(crate) level: Option<String>
}


#[derive(Deserialize, Debug, Clone)]
pub struct KubeConfig {

}

#[derive(Deserialize, Debug, Clone)]
pub struct CustomConfig {
    title: Option<String>,
    platform_name: String,
}